use hyper::header;

use super::{Token, Lifetime};

/// The bearer token type.
///
/// See [RFC6750](http://tools.ietf.org/html/rfc6750).
pub struct Bearer<L: Lifetime> {
    access_token: String,
    scope: Option<String>,
    lifetime: L,
}

impl<L: Lifetime> Token<L> for Bearer<L> {
    fn access_token(&self) -> &str { &self.access_token }
    fn scope(&self) -> Option<&str> { self.scope.as_ref().map(|s| &s[..]) }
    fn lifetime(&self) -> &L { &self.lifetime }
}

impl<'a, L: Lifetime> Into<header::Authorization<header::Bearer>> for &'a Bearer<L> {
    fn into(self) -> header::Authorization<header::Bearer> {
        header::Authorization(header::Bearer { token: self.access_token.clone() })
    }
}
