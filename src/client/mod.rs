//! Client.

mod error;

pub mod response;
pub use self::error::ClientError;

use std::marker::PhantomData;

use futures::future::{Future, IntoFuture};
use futures::Stream;
use hyper::{self, header, mime, Request };
use hyper_rustls::HttpsConnector;
use serde_json::{self, Value};
use url::Url;
use url::form_urlencoded::Serializer;

use client::response::FromResponse;
use provider::Provider;
use token::{Token, Lifetime, Refresh};

/// OAuth 2.0 client.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client<P: Provider> {
    /// Client ID.
    pub client_id: String,

    /// Client secret.
    pub client_secret: String,

    /// Redirect URI.
    // TODO Make an actual uri
    pub redirect_uri: Option<String>,

    provider: PhantomData<P>,
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
    /// let client = Client::<Installed>::new(
    ///     String::from("CLIENT_ID"),
    ///     String::from("CLIENT_SECRET"),
    ///     Some(String::from("urn:ietf:wg:oauth:2.0:oob"))
    /// );
    /// ```
    pub fn new(client_id: String, client_secret: String, redirect_uri: Option<String>) -> Self {
        Client {
            client_id: client_id,
            client_secret: client_secret,
            redirect_uri: redirect_uri,
            provider: PhantomData,
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
    /// let client = Client::<Installed>::new(
    ///     String::from("CLIENT_ID"),
    ///     String::from("CLIENT_SECRET"),
    ///     Some(String::from("urn:ietf:wg:oauth:2.0:oob"))
    /// );
    ///
    /// let auth_uri = client.auth_uri(
    ///     Some("https://www.googleapis.com/auth/userinfo.email"),
    ///     None
    /// );
    /// ```
    pub fn auth_uri(&self, scope: Option<&str>, state: Option<&str>) -> Result<Url, ClientError>
    {
        // TODO uris should be uris and not need parsing eventually
        let mut uri = Url::parse(P::auth_uri())?;
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
        Ok(uri)
    }

    fn post_token(
        &self,
        http_client: &hyper::Client<HttpsConnector>,
        mut body: Serializer<String>
    ) -> impl Future<Item=Value, Error=ClientError> {
        if P::credentials_in_body() {
            body.append_pair("client_id", &self.client_id);
            body.append_pair("client_secret", &self.client_secret);
        }

        // TODO We should be packing the auth / token uris into uris from the start
        let mut request = Request::new(hyper::Method::Post, P::token_uri().parse().unwrap());

        request.headers_mut().set(header::Authorization(
            header::Basic {
                username: self.client_id.clone(),
                password: Some(self.client_secret.clone()),
            }));
        request.headers_mut().set(header::Accept(vec![ header::qitem(mime::APPLICATION_JSON), ]));
        request.headers_mut().set(header::ContentType::form_url_encoded());

        request.set_body(body.finish());

        http_client.request(request).map_err(|e| ClientError::from(e))
            .and_then(|res| { res.body().concat2()
            .map_err(|e| ClientError::from(e))
                .and_then(move |body| { serde_json::from_slice(&body)
                .map_err(|e| ClientError::from(e))
            })
        })
    }

    /// Requests an access token using an authorization code.
    ///
    /// See [RFC 6749, section 4.1.3](http://tools.ietf.org/html/rfc6749#section-4.1.3).
    pub fn request_token(
        &self,
        http_client: &hyper::Client<HttpsConnector>,
        code: &str
    ) -> impl Future<Item=P::Token, Error=ClientError> {
        let mut body = Serializer::new(String::new());
        body.append_pair("grant_type", "authorization_code");
        body.append_pair("code", code);

        if let Some(ref redirect_uri) = self.redirect_uri {
            body.append_pair("redirect_uri", redirect_uri);
        }

        self.post_token(http_client, body).and_then(move |json| {
            P::Token::from_response(&json).map_err(|e| ClientError::from(e))
        })
    }
}

impl<P: Provider> Client<P> where P::Token: Token<Refresh> {
    /// Refreshes an access token.
    ///
    /// See [RFC 6749, section 6](http://tools.ietf.org/html/rfc6749#section-6).
    pub fn refresh_token(
        &self,
        http_client: &hyper::Client<HttpsConnector>,
        token: P::Token,
        scope: Option<&str>
    ) -> impl Future<Item=P::Token, Error=ClientError> {
        let mut body = Serializer::new(String::new());
        body.append_pair("grant_type", "refresh_token");
        body.append_pair("refresh_token", token.lifetime().refresh_token());

        if let Some(scope) = scope {
            body.append_pair("scope", scope);
        }

        self.post_token(http_client, body).and_then(move |json| {
            P::Token::from_response_inherit(&json, &token).map_err(|e| ClientError::from(e))
        })
    }

    /// Ensures an access token is valid by refreshing it if necessary.
    pub fn ensure_token(&self, http_client: &hyper::Client<HttpsConnector>, token: P::Token) 
    -> Box<Future<Item=P::Token, Error=ClientError>> where P: 'static {
        if token.lifetime().expired() {
            Box::new(self.refresh_token(http_client, token, None))
        } else {
            Box::new(Ok(token).into_future())
        }
    }
}

#[cfg(test)]
mod tests {
    use token::{Bearer, Static};
    use provider::Provider;
    use super::Client;

    struct Test;
    impl Provider for Test {
        type Lifetime = Static;
        type Token = Bearer<Static>;
        fn auth_uri() -> &'static str { "http://example.com/oauth2/auth" }
        fn token_uri() -> &'static str { "http://example.com/oauth2/token" }
    }

    #[test]
    fn auth_uri() {
        let client = Client::<Test>::new(String::from("foo"), String::from("bar"), None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo",
            client.auth_uri(None, None).unwrap().as_str()
        );
    }

    #[test]
    fn auth_uri_with_redirect_uri() {
        let client = Client::<Test>::new(
            String::from("foo"),
            String::from("bar"),
            Some(String::from("http://example.com/oauth2/callback"))
        );
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&redirect_uri=http%3A%2F%2Fexample.com%2Foauth2%2Fcallback",
            client.auth_uri(None, None).unwrap().as_str()
        );
    }

    #[test]
    fn auth_uri_with_scope() {
        let client = Client::<Test>::new(String::from("foo"), String::from("bar"), None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&scope=baz",
            client.auth_uri(Some("baz"), None).unwrap().as_str()
        );
    }

    #[test]
    fn auth_uri_with_state() {
        let client = Client::<Test>::new(String::from("foo"), String::from("bar"), None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&state=baz",
            client.auth_uri(None, Some("baz")).unwrap().as_str()
        );
    }
}
