use std::io::Read;

use chrono::{UTC, Duration};
use hyper::{self, header, mime};
use rustc_serialize::json;
use url::{Url, form_urlencoded};

use super::Token;
use super::error::{Error, Result, OAuth2Error, OAuth2ErrorCode};

/// OAuth 2.0 client.
pub struct Client {
    http_client: hyper::Client,

    auth_uri: String,
    token_uri: String,

    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
}

#[derive(RustcDecodable)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

impl Into<Token> for TokenResponse {
    fn into(self) -> Token {
        Token {
            access_token: self.access_token,
            token_type: self.token_type,
            expires: self.expires_in.map(|s| UTC::now() + Duration::seconds(s)),
            refresh_token: self.refresh_token,
            scope: self.scope,
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

macro_rules! site_constructors {
    (
        $(
            #[$attr:meta]
            $ident:ident => ($auth_uri:expr, $token_uri:expr)
        ),*
    ) => {
        $(
            #[$attr]
            pub fn $ident<S>(
                http_client: hyper::Client,
                client_id: S,
                client_secret: S,
                redirect_uri: Option<S>
            ) -> Self where S: Into<String> {
                Client {
                    http_client: http_client,
                    auth_uri: String::from($auth_uri),
                    token_uri: String::from($token_uri),
                    client_id: client_id.into(),
                    client_secret: client_secret.into(),
                    redirect_uri: redirect_uri.map(Into::into),
                }
            }
        )*
    }
}

impl Client {
    /// Creates an OAuth 2.0 client.
    pub fn new<S>(
        http_client: hyper::Client,
        auth_uri: S,
        token_uri: S,
        client_id: S,
        client_secret: S,
        redirect_uri: Option<S>
    ) -> Self where S: Into<String> {
        Client {
            http_client: http_client,
            auth_uri: auth_uri.into(),
            token_uri: token_uri.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.map(Into::into),
        }
    }

    site_constructors!{
        #[doc = "Creates a Google OAuth 2.0 client."]
        google => (
            "https://accounts.google.com/o/oauth2/auth",
            "https://accounts.google.com/o/oauth2/token"
        ),

        #[doc = "Creates a GitHub OAuth 2.0 client."]
        github => (
            "https://github.com/login/oauth/authorize",
            "https://github.com/login/oauth/access_token"
        )
    }

    /// Constructs an authorization request URI.
    pub fn auth_uri(&self, scope: Option<&str>, state: Option<&str>) -> Result<String> {
        let mut uri = try!(Url::parse(&self.auth_uri));

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

        uri.set_query_from_pairs(query_pairs.iter());

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

    fn token_post(&self, body_pairs: Vec<(&str, &str)>) -> Result<Token> {
        let post_body = form_urlencoded::serialize(body_pairs);
        let request = self.http_client.post(&self.token_uri)
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
    pub fn request_token(&self, code: &str) -> Result<Token> {
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
    /// # Panics
    ///
    /// Panics if `token` does not contain a `refresh_token`.
    pub fn refresh_token(&self, token: &Token, scope: Option<&str>) -> Result<Token> {
        let refresh_token = token.refresh_token.as_ref().unwrap();

        let mut body_pairs = vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        if let Some(scope) = scope {
            body_pairs.push(("scope", scope));
        }

        let mut result = self.token_post(body_pairs);

        if let Ok(ref mut token) = result {
            if token.refresh_token.is_none() {
                token.refresh_token = Some(refresh_token.clone());
            }
        }

        result
    }
}
