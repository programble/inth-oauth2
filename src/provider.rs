//! Providers.

use token::{Token, Lifetime, Bearer, Expiring};

/// OAuth 2.0 providers.
pub trait Provider {
    /// The lifetime of tokens issued by the provider.
    type Lifetime: Lifetime;

    /// The type of token issued by the provider.
    type Token: Token<Self::Lifetime>;

    /// The authorization endpoint URI.
    ///
    /// See [RFC 6749, section 3.1](http://tools.ietf.org/html/rfc6749#section-3.1).
    fn auth_uri() -> &'static str;

    /// The token endpoint URI.
    ///
    /// See [RFC 6749, section 3.2](http://tools.ietf.org/html/rfc6749#section-3.2).
    fn token_uri() -> &'static str;
}

/// Google OAuth 2.0 provider.
///
/// See [Using OAuth 2.0 to Access Google
/// APIs](https://developers.google.com/identity/protocols/OAuth2).
pub struct Google;
impl Provider for Google {
    type Lifetime = Expiring;
    type Token = Bearer<Expiring>;
    fn auth_uri() -> &'static str { "https://accounts.google.com/o/oauth2/v2/auth" }
    fn token_uri() -> &'static str { "https://www.googleapis.com/oauth2/v4/token" }
}
