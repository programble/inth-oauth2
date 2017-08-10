//! Providers.

use token::{Token, Lifetime, Bearer, Static, Refresh};

/// OAuth 2.0 providers.
pub trait Provider {
    /// The lifetime of tokens issued by the provider.
    type Lifetime: Lifetime;

    /// The type of token issued by the provider.
    type Token: Token<Self::Lifetime>;

    /// The authorization endpoint URI.
    ///
    /// See [RFC 6749, section 3.1](http://tools.ietf.org/html/rfc6749#section-3.1).
    ///
    /// Note: likely to become an associated constant.
    fn auth_uri() -> &'static str;

    /// The token endpoint URI.
    ///
    /// See [RFC 6749, section 3.2](http://tools.ietf.org/html/rfc6749#section-3.2).
    ///
    /// Note: likely to become an associated constant.
    fn token_uri() -> &'static str;

    /// Provider requires credentials via request body.
    ///
    /// Although not recommended by the RFC, some providers require `client_id` and `client_secret`
    /// as part of the request body.
    ///
    /// See [RFC 6749, section 2.3.1](http://tools.ietf.org/html/rfc6749#section-2.3.1).
    ///
    /// Note: likely to become an associated constant.
    fn credentials_in_body() -> bool {
        false
    }
}

/// Google OAuth 2.0 providers.
///
/// See [Using OAuth 2.0 to Access Google
/// APIs](https://developers.google.com/identity/protocols/OAuth2).
pub mod google {
    use token::{Bearer, Expiring, Refresh};
    use super::Provider;

    /// Signals the server to return the authorization code by prompting the user to copy and
    /// paste.
    ///
    /// See [Choosing a redirect URI][uri].
    ///
    /// [uri]: https://developers.google.com/identity/protocols/OAuth2InstalledApp#choosingredirecturi
    pub const REDIRECT_URI_OOB: &'static str = "urn:ietf:wg:oauth:2.0:oob";

    /// Signals the server to return the authorization code in the page title.
    ///
    /// See [Choosing a redirect URI][uri].
    ///
    /// [uri]: https://developers.google.com/identity/protocols/OAuth2InstalledApp#choosingredirecturi
    pub const REDIRECT_URI_OOB_AUTO: &'static str = "urn:ietf:wg:oauth:2.0:oob:auto";

    /// Google OAuth 2.0 provider for web applications.
    ///
    /// See [Using OAuth 2.0 for Web Server
    /// Applications](https://developers.google.com/identity/protocols/OAuth2WebServer).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Web;
    impl Provider for Web {
        type Lifetime = Expiring;
        type Token = Bearer<Expiring>;
        fn auth_uri() -> &'static str {
            "https://accounts.google.com/o/oauth2/v2/auth"
        }
        fn token_uri() -> &'static str {
            "https://www.googleapis.com/oauth2/v4/token"
        }
    }

    /// Google OAuth 2.0 provider for installed applications.
    ///
    /// See [Using OAuth 2.0 for Installed
    /// Applications](https://developers.google.com/identity/protocols/OAuth2InstalledApp).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Installed;
    impl Provider for Installed {
        type Lifetime = Refresh;
        type Token = Bearer<Refresh>;
        fn auth_uri() -> &'static str {
            "https://accounts.google.com/o/oauth2/v2/auth"
        }
        fn token_uri() -> &'static str {
            "https://www.googleapis.com/oauth2/v4/token"
        }
    }
}

/// GitHub OAuth 2.0 provider.
///
/// See [OAuth, GitHub Developer Guide](https://developer.github.com/v3/oauth/).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitHub;
impl Provider for GitHub {
    type Lifetime = Static;
    type Token = Bearer<Static>;
    fn auth_uri() -> &'static str {
        "https://github.com/login/oauth/authorize"
    }
    fn token_uri() -> &'static str {
        "https://github.com/login/oauth/access_token"
    }
}

/// Imgur OAuth 2.0 provider.
///
/// See [OAuth 2.0, Imgur](https://api.imgur.com/oauth2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imgur;
impl Provider for Imgur {
    type Lifetime = Refresh;
    type Token = Bearer<Refresh>;
    fn auth_uri() -> &'static str {
        "https://api.imgur.com/oauth2/authorize"
    }
    fn token_uri() -> &'static str {
        "https://api.imgur.com/oauth2/token"
    }
}
