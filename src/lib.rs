extern crate url;

use url::{Url, ParseResult};

/// OAuth2 client.
pub struct Client {
    pub authorization_uri: String,

    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>,
}

impl Client {
    pub fn authorization_uri(&self, scope: Option<&str>, state: Option<&str>) -> ParseResult<String> {
        let mut uri = try!(Url::parse(&self.authorization_uri));

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
