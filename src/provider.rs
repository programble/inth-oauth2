//! Providers.

use token::{Token, Lifetime};

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
