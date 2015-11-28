//! OAuth2 client.

use url::{Url, ParseResult};

/// OAuth2 client.
pub struct Client {
    auth_uri: String,
    token_uri: String,

    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
}

impl Client {
    pub fn new<S: Into<String>>(
        auth_uri: S,
        token_uri: S,
        client_id: S,
        client_secret: S,
        redirect_uri: Option<S>
    ) -> Self {
        Client {
            auth_uri: auth_uri.into(),
            token_uri: token_uri.into(),

            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.map(Into::<String>::into),
        }
    }

    pub fn google<S: Into<String>>(
        client_id: S,
        client_secret: S,
        redirect_uri: Option<S>
    ) -> Self {
        Client {
            auth_uri: String::from("https://accounts.google.com/o/oauth2/auth"),
            token_uri: String::from("https://accounts.google.com/o/oauth2/token"),

            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.map(Into::<String>::into),
        }
    }

    pub fn auth_uri(&self, scope: Option<&str>, state: Option<&str>) -> ParseResult<String> {
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
}
