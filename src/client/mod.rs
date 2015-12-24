//! Client.

use std::fmt;
use std::marker::PhantomData;

use hyper::{self, header, mime};
use rustc_serialize::json::Json;
use url::{self, form_urlencoded, Url};

use error::OAuth2Error;
use provider::Provider;

use self::response::FromResponse;
pub mod response;

pub use self::error::ClientError;
mod error;

/// OAuth 2.0 client.
pub struct Client<P: Provider> {
    http_client: hyper::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
    provider: PhantomData<P>,
}

impl<P: Provider> fmt::Debug for Client<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Client")
            .field("client_id", &self.client_id)
            .field("client_secret", &self.client_secret)
            .field("redirect_uri", &self.redirect_uri)
            .finish()
    }
}

impl<P: Provider> Client<P> {
    /// Creates a client.
    ///
    /// # Examples
    ///
    /// ```
    /// use inth_oauth2::client::Client;
    /// use inth_oauth2::provider::Google;
    ///
    /// let client = Client::<Google>::new(
    ///     Default::default(),
    ///     "CLIENT_ID",
    ///     "CLIENT_SECRET",
    ///     Some("urn:ietf:wg:oauth:2.0:oob")
    /// );
    /// ```
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

    /// Returns an authorization endpoint URI to direct the user to.
    ///
    /// See [RFC 6749, section 3.1](http://tools.ietf.org/html/rfc6749#section-3.1).
    ///
    /// # Examples
    ///
    /// ```
    /// use inth_oauth2::client::Client;
    /// use inth_oauth2::provider::Google;
    ///
    /// let client = Client::<Google>::new(
    ///     Default::default(),
    ///     "CLIENT_ID",
    ///     "CLIENT_SECRET",
    ///     Some("urn:ietf:wg:oauth:2.0:oob")
    /// );
    ///
    /// let auth_uri = client.auth_uri(
    ///     Some("https://www.googleapis.com/auth/userinfo.email"),
    ///     None
    /// );
    /// ```
    pub fn auth_uri(&self, scope: Option<&str>, state: Option<&str>) -> Result<String, url::ParseError>
    {
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

        uri.set_query_from_pairs(query_pairs.iter());

        Ok(uri.serialize())
    }

    /// Requests an access token using an authorization code.
    ///
    /// See [RFC 6749, section 4.1.3](http://tools.ietf.org/html/rfc6749#section-4.1.3).
    pub fn request_token(&self, code: &str) -> Result<P::Token, ClientError> {
        let mut body_pairs = vec![
            ("grant_type", "authorization_code"),
            ("code", code),
        ];
        if let Some(ref redirect_uri) = self.redirect_uri {
            body_pairs.push(("redirect_uri", redirect_uri));
        }

        let post_body = form_urlencoded::serialize(body_pairs);
        let request = self.http_client.post(P::token_uri())
            .header(
                header::Authorization(header::Basic {
                    username: self.client_id.clone(),
                    password: Some(self.client_secret.clone()),
                })
            )
            .header(
                header::Accept(vec![
                    header::qitem(
                        mime::Mime(
                            mime::TopLevel::Application,
                            mime::SubLevel::Json,
                            vec![]
                        )
                    )
                ])
            )
            .header(header::ContentType::form_url_encoded())
            .body(&post_body);

        let mut response = try!(request.send());
        let json = try!(Json::from_reader(&mut response));

        let error = OAuth2Error::from_response(&json);
        if let Ok(error) = error {
            return Err(ClientError::from(error));
        }

        let token = try!(P::Token::from_response(&json));
        Ok(token)
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
        let client = Client::<Test>::new(Default::default(), "foo", "bar", None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo",
            client.auth_uri(None, None).unwrap()
        );
    }

    #[test]
    fn auth_uri_with_redirect_uri() {
        let client = Client::<Test>::new(
            Default::default(),
            "foo",
            "bar",
            Some("http://example.com/oauth2/callback")
        );
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&redirect_uri=http%3A%2F%2Fexample.com%2Foauth2%2Fcallback",
            client.auth_uri(None, None).unwrap()
        );
    }

    #[test]
    fn auth_uri_with_scope() {
        let client = Client::<Test>::new(Default::default(), "foo", "bar", None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&scope=baz",
            client.auth_uri(Some("baz"), None).unwrap()
        );
    }

    #[test]
    fn auth_uri_with_state() {
        let client = Client::<Test>::new(Default::default(), "foo", "bar", None);
        assert_eq!(
            "http://example.com/oauth2/auth?response_type=code&client_id=foo&state=baz",
            client.auth_uri(None, Some("baz")).unwrap()
        );
    }
}
