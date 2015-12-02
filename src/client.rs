use std::io::Read;
use std::marker::PhantomData;

use chrono::{UTC, Duration};
use hyper::{self, header, mime};
use rustc_serialize::json;
use url::{Url, form_urlencoded};

use super::Provider;
use super::{TokenPair, AccessTokenType, AccessToken, RefreshToken};
use super::error::{Error, Result, OAuth2Error, OAuth2ErrorCode};

/// OAuth 2.0 client.
///
/// Performs HTTP requests using the provided `hyper::Client`.
///
/// See [RFC6749 section 4.1](http://tools.ietf.org/html/rfc6749#section-4.1).
pub struct Client<P: Provider> {
    http_client: hyper::Client,

    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,

    provider: PhantomData<P>,
}

impl<P: Provider> Client<P> {
    /// Creates an OAuth 2.0 client.
    pub fn new<S>(
        http_client: hyper::Client,
        client_id: S,
        client_secret: S,
        redirect_uri: Option<S>
    ) -> Self where S: Into<String> {
        Client {
            http_client: http_client,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.map(Into::into),
            provider: PhantomData,
        }
    }
}

#[derive(RustcDecodable)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

impl Into<TokenPair> for TokenResponse {
    fn into(self) -> TokenPair {
        TokenPair {
            access: AccessToken {
                token: self.access_token,
                token_type: match &self.token_type[..] {
                    "Bearer" | "bearer" => AccessTokenType::Bearer,
                    _ => AccessTokenType::Unrecognized(self.token_type),
                },
                expires: self.expires_in.map(|s| UTC::now() + Duration::seconds(s)),
                scope: self.scope,
            },
            refresh: self.refresh_token.map(|t| RefreshToken { token: t }),
        }
    }
}

#[derive(RustcDecodable)]
struct ErrorResponse {
    error: String,
    error_description: Option<String>,
    error_uri: Option<String>,
}

impl Into<OAuth2Error> for ErrorResponse {
    fn into(self) -> OAuth2Error {
        let code = match &self.error[..] {
            "invalid_request" => OAuth2ErrorCode::InvalidRequest,
            "invalid_client" => OAuth2ErrorCode::InvalidClient,
            "invalid_grant" => OAuth2ErrorCode::InvalidGrant,
            "unauthorized_client" => OAuth2ErrorCode::UnauthorizedClient,
            "unsupported_grant_type" => OAuth2ErrorCode::UnsupportedGrantType,
            "invalid_scope" => OAuth2ErrorCode::InvalidScope,
            _ => OAuth2ErrorCode::Unrecognized(self.error),
        };
        OAuth2Error {
            code: code,
            description: self.error_description,
            uri: self.error_uri,
        }
    }
}

impl<P: Provider> Client<P> {
    /// Constructs an authorization request URI.
    ///
    /// See [RFC6749 section 4.1.1](http://tools.ietf.org/html/rfc6749#section-4.1.1).
    pub fn auth_uri(&self, scope: Option<&str>, state: Option<&str>) -> Result<String> {
        let mut uri = try!(Url::parse(P::auth_uri()));

        let mut query_pairs = vec![
            ("response_type", "code"),
            ("client_id", &self.client_id),
        ];
        if let Some(ref redirect_uri) = self.redirect_uri {
            query_pairs.push(("redirect_uri", redirect_uri));
        }
        if let Some(scope) = scope {
            query_pairs.push(("scope", scope));
        }
        if let Some(state) = state {
            query_pairs.push(("state", state));
        }

        uri.set_query_from_pairs(query_pairs);

        Ok(uri.serialize())
    }

    fn auth_header(&self) -> header::Authorization<header::Basic> {
        header::Authorization(
            header::Basic {
                username: self.client_id.clone(),
                password: Some(self.client_secret.clone()),
            }
        )
    }

    fn accept_header(&self) -> header::Accept {
        header::Accept(vec![
            header::qitem(
                mime::Mime(
                    mime::TopLevel::Application,
                    mime::SubLevel::Json,
                    vec![]
                )
            ),
        ])
    }

    fn token_post(&self, body_pairs: Vec<(&str, &str)>) -> Result<TokenPair> {
        let post_body = form_urlencoded::serialize(body_pairs);
        let request = self.http_client.post(P::token_uri())
            .header(self.auth_header())
            .header(self.accept_header())
            .header(header::ContentType::form_url_encoded())
            .body(&post_body);

        let mut response = try!(request.send());
        let mut body = String::new();
        try!(response.read_to_string(&mut body));

        let token = json::decode::<TokenResponse>(&body);
        if let Ok(token) = token {
            return Ok(token.into());
        }

        let error: ErrorResponse = try!(json::decode(&body));
        Err(Error::OAuth2(error.into()))
    }

    /// Requests an access token using an authorization code.
    ///
    /// See [RFC6749 section 4.1.3](http://tools.ietf.org/html/rfc6749#section-4.1.3).
    pub fn request_token(&self, code: &str) -> Result<TokenPair> {
        let mut body_pairs = vec![
            ("grant_type", "authorization_code"),
            ("code", code),
        ];
        if let Some(ref redirect_uri) = self.redirect_uri {
            body_pairs.push(("redirect_uri", redirect_uri));
        }
        self.token_post(body_pairs)
    }

    /// Refreshes an access token.
    ///
    /// The returned `TokenPair` will always have a `refresh`.
    ///
    /// See [RFC6749 section 6](http://tools.ietf.org/html/rfc6749#section-6).
    pub fn refresh_token(&self, refresh: RefreshToken, scope: Option<&str>) -> Result<TokenPair> {
        let mut result = {
            let mut body_pairs = vec![
                ("grant_type", "refresh_token"),
                ("refresh_token", &refresh.token),
            ];
            if let Some(scope) = scope {
                body_pairs.push(("scope", scope));
            }

            self.token_post(body_pairs)
        };

        if let Ok(ref mut pair) = result {
            if pair.refresh.is_none() {
                pair.refresh = Some(refresh);
            }
        }

        result
    }
}
