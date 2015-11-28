use std::io::Read;

use chrono::{UTC, Duration};
use hyper::{self, header, mime};
use rustc_serialize::json;
use url::{Url, form_urlencoded};

use super::{Error, Result, Token};

/// OAuth 2.0 client.
pub struct Client {
    http_client: hyper::Client,

    auth_uri: String,
    token_uri: String,

    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
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
            pub fn $ident<S: Into<String>>(
                http_client: hyper::Client,
                client_id: S,
                client_secret: S,
                redirect_uri: Option<S>
            ) -> Self {
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
    pub fn new<S: Into<String>>(
        http_client: hyper::Client,
        auth_uri: S,
        token_uri: S,
        client_id: S,
        client_secret: S,
        redirect_uri: Option<S>
    ) -> Self {
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

    /// Requests an access token using an authorization code.
    pub fn request_token(&self, code: &str) -> Result<Token> {
        let auth_header = header::Authorization(
            header::Basic {
                username: self.client_id.clone(),
                password: Some(self.client_secret.clone()),
            }
        );

        let accept_header = header::Accept(vec![
            header::qitem(
                mime::Mime(
                    mime::TopLevel::Application,
                    mime::SubLevel::Json,
                    vec![]
                )
            ),
        ]);

        let mut body_pairs = vec![
            ("grant_type", "authorization_code"),
            ("code", code),
        ];
        if let Some(ref redirect_uri) = self.redirect_uri {
            body_pairs.push(("redirect_uri", redirect_uri));
        }

        let body_str = form_urlencoded::serialize(body_pairs);

        let request = self.http_client.post(&self.token_uri)
            .header(auth_header)
            .header(accept_header)
            .header(header::ContentType::form_url_encoded())
            .body(&body_str);

        let mut response = try!(request.send());
        let mut json = String::new();
        try!(response.read_to_string(&mut json));

        if response.status != hyper::Ok {
            return Err(Error::Todo);
        }

        #[derive(RustcDecodable)]
        struct TokenResponse {
            access_token: String,
            token_type: String,
            expires_in: Option<i64>,
            refresh_token: Option<String>,
            scope: Option<String>,
        }
        let token: TokenResponse = try!(json::decode(&json));

        Ok(Token {
            access_token: token.access_token,
            token_type: token.token_type,
            expires: token.expires_in.map(|s| UTC::now() + Duration::seconds(s)),
            refresh_token: token.refresh_token,
            scope: token.scope,
        })
    }
}
