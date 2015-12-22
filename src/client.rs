//! Client.

use std::marker::PhantomData;

use hyper;
use url::{self, form_urlencoded, Url};

use provider::Provider;

/// OAuth 2.0 client.
pub struct Client<P: Provider> {
    http_client: hyper::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
    provider: PhantomData<P>,
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
}
