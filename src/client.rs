//! Client.

use std::marker::PhantomData;

use hyper;

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
}
