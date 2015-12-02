/// An OAuth 2.0 provider.
pub trait Provider {
    /// The authorization endpoint URI.
    fn auth_uri() -> &'static str;

    /// The token endpoint URI.
    fn token_uri() -> &'static str;
}

/// Google OAuth 2.0 provider.
///
/// See [Using OAuth 2.0 to Access Google
/// APIs](https://developers.google.com/identity/protocols/OAuth2).
pub struct Google;
impl Provider for Google {
    fn auth_uri() -> &'static str { "https://accounts.google.com/o/oauth2/auth" }
    fn token_uri() -> &'static str { "https://accounts.google.com/o/oauth2/token" }
}

/// GitHub OAuth 2.0 provider.
///
/// See [OAuth, GitHub API](https://developer.github.com/v3/oauth/).
pub struct GitHub;
impl Provider for GitHub {
    fn auth_uri() -> &'static str { "https://github.com/login/oauth/authorize" }
    fn token_uri() -> &'static str { "https://github.com/login/oauth/access_token" }
}

/// Imgur OAuth 2.0 provider.
///
/// See [OAuth 2.0, Imgur](https://api.imgur.com/oauth2).
pub struct Imgur;
impl Provider for Imgur {
    fn auth_uri() -> &'static str { "https://api.imgur.com/oauth2/authorize" }
    fn token_uri() -> &'static str { "https://api.imgur.com/oauth2/token" }
}
