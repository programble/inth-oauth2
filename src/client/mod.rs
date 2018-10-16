//! Client.

mod error;

pub mod response;
pub use self::error::ClientError;

use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::{self, Value};
use url::form_urlencoded::Serializer;
use url::Url;

use client::response::FromResponse;
use error::OAuth2Error;
use provider::Provider;
use token::{Lifetime, Refresh, Token};

/// OAuth 2.0 client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client<P> {
    /// OAuth provider.
    pub provider: P,

    /// Client ID.
    pub client_id: String,

    /// Client secret.
    pub client_secret: String,

    /// Redirect URI.
    pub redirect_uri: Option<String>,
}

impl<P: Provider> Client<P> {
    /// Creates a client.
    ///
    /// # Examples
    ///
    /// ```
    /// use inth_oauth2::Client;
    /// use inth_oauth2::provider::google::Installed;
    ///
    /// let client = Client::new(
    ///     Installed,
    ///     String::from("CLIENT_ID"),
    ///     String::from("CLIENT_SECRET"),
    ///     Some(String::from("urn:ietf:wg:oauth:2.0:oob")),
    /// );
    /// ```
    pub fn new(
        provider: P,
        client_id: String,
        client_secret: String,
        redirect_uri: Option<String>,
    ) -> Self {
        Client {
            provider,
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    /// Returns an authorization endpoint URI to direct the user to.
    ///
    /// See [RFC 6749, section 3.1](http://tools.ietf.org/html/rfc6749#section-3.1).
    ///
    /// # Examples
    ///
    /// ```
    /// use inth_oauth2::Client;
    /// use inth_oauth2::provider::google::Installed;
    ///
    /// let client = Client::new(
    ///     Installed,
    ///     String::from("CLIENT_ID"),
    ///     String::from("CLIENT_SECRET"),
    ///     Some(String::from("urn:ietf:wg:oauth:2.0:oob")),
    /// );
    ///
    /// let auth_uri = client.auth_uri(
    ///     Some("https://www.googleapis.com/auth/userinfo.email"),
    ///     None,
    /// );
    /// ```
    pub fn auth_uri(&self, scope: Option<&str>, state: Option<&str>) -> Url
    {
        let mut uri = self.provider.auth_uri().clone();

        {
            let mut query = uri.query_pairs_mut();

            query.append_pair("response_type", "code");
            query.append_pair("client_id", &self.client_id);

            if let Some(ref redirect_uri) = self.redirect_uri {
                query.append_pair("redirect_uri", redirect_uri);
            }
            if let Some(scope) = scope {
                query.append_pair("scope", scope);
            }
            if let Some(state) = state {
                query.append_pair("state", state);
            }
        }

        uri
    }

    fn post_token(
        &self,
        http_client: &reqwest::Client,
        mut body: Serializer<String>,
    ) -> Result<Value, ClientError> {
        if self.provider.credentials_in_body() {
            body.append_pair("client_id", &self.client_id);
            body.append_pair("client_secret", &self.client_secret);
        }

        let body = body.finish();

        let mut response = http_client
            .post(self.provider.token_uri().clone())
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()?;

        let json = serde_json::from_reader(&mut response)?;

        let error = OAuth2Error::from_response(&json);

        if let Ok(error) = error {
            Err(ClientError::from(error))
        } else {
            Ok(json)
        }
    }

    /// Requests an access token using an authorization code.
    ///
    /// See [RFC 6749, section 4.1.3](http://tools.ietf.org/html/rfc6749#section-4.1.3).
    pub fn request_token(
        &self,
        http_client: &reqwest::Client,
        code: &str,
    ) -> Result<P::Token, ClientError> {
        let mut body = Serializer::new(String::new());
        body.append_pair("grant_type", "authorization_code");
        body.append_pair("code", code);

        if let Some(ref redirect_uri) = self.redirect_uri {
            body.append_pair("redirect_uri", redirect_uri);
        }

        let json = self.post_token(http_client, body)?;
        let token = P::Token::from_response(&json)?;
        Ok(token)
    }
}

impl<P> Client<P> where P: Provider, P::Token: Token<Refresh> {
    /// Refreshes an access token.
    ///
    /// See [RFC 6749, section 6](http://tools.ietf.org/html/rfc6749#section-6).
    pub fn refresh_token(
        &self,
        http_client: &reqwest::Client,
        token: P::Token,
        scope: Option<&str>,
    ) -> Result<P::Token, ClientError> {
        let mut body = Serializer::new(String::new());
        body.append_pair("grant_type", "refresh_token");
        body.append_pair("refresh_token", token.lifetime().refresh_token());

        if let Some(scope) = scope {
            body.append_pair("scope", scope);
        }

        let json = self.post_token(http_client, body)?;
        let token = P::Token::from_response_inherit(&json, &token)?;
        Ok(token)
    }

    /// Ensures an access token is valid by refreshing it if necessary.
    pub fn ensure_token(
        &self,
        http_client: &reqwest::Client,
        token: P::Token,
    ) -> Result<P::Token, ClientError> {
        if token.lifetime().expired() {
            self.refresh_token(http_client, token, None)
        } else {
            Ok(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;
    use token::{Bearer, Static};
    use provider::Provider;
    use super::Client;

    struct Test {
        auth_uri: Url,
        token_uri: Url
    }
    impl Provider for Test {
        type Lifetime = Static;
        type Token = Bearer<Static>;
        fn auth_uri(&self) -> &Url { &self.auth_uri }
        fn token_uri(&self) -> &Url { &self.token_uri }
    }
    impl Test {
        fn new() -> Self {
            Test {
                auth_uri: Url::parse("http://example.com/oauth2/auth").unwrap(),
                token_uri: Url::parse("http://example.com/oauth2/token").unwrap()
            }
        }
    }

    #[test]
    fn auth_uri() {
        let client = Client::new(Test::new(), String::from("foo"), String::from("bar"), None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo",
            client.auth_uri(None, None).as_str()
        );
    }

    #[test]
    fn auth_uri_with_redirect_uri() {
        let client = Client::new(
            Test::new(),
            String::from("foo"),
            String::from("bar"),
            Some(String::from("http://example.com/oauth2/callback")),
        );
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&redirect_uri=http%3A%2F%2Fexample.com%2Foauth2%2Fcallback",
            client.auth_uri(None, None).as_str()
        );
    }

    #[test]
    fn auth_uri_with_scope() {
        let client = Client::new(Test::new(), String::from("foo"), String::from("bar"), None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&scope=baz",
            client.auth_uri(Some("baz"), None).as_str()
        );
    }

    #[test]
    fn auth_uri_with_state() {
        let client = Client::new(Test::new(), String::from("foo"), String::from("bar"), None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&state=baz",
            client.auth_uri(None, Some("baz")).as_str()
        );
    }
}
