//! # "It's not that hard" OAuth2 Client
//!
//! OAuth2 really isn't that hard, you know?
//!
//! `inth_oauth2` is on [Crates.io][crate] and [GitHub][github].
//!
//! [crate]: https://crates.io/crates/inth-oauth2
//! [github]: https://github.com/programble/inth-oauth2
//!
//! Implementation of [RFC6749](http://tools.ietf.org/html/rfc6749).
//!
//! ## Providers
//!
//! `inth_oauth2` can be used with any OAuth 2.0 provider, but provides defaults for a few common
//! ones.
//!
//! ### Google
//!
//! ```
//! use inth_oauth2::Client as OAuth2;
//!
//! let auth = OAuth2::google(
//!     Default::default(),
//!     "CLIENT_ID",
//!     "CLIENT_SECRET",
//!     Some("REDIRECT_URI")
//! );
//! ```
//!
//! ### GitHub
//!
//! ```
//! use inth_oauth2::Client as OAuth2;
//!
//! let auth = OAuth2::github(Default::default(), "CLIENT_ID", "CLIENT_SECRET", None);
//! ```
//!
//! ### Other
//!
//! An authorization URI and a token URI are required.
//!
//! ```
//! use inth_oauth2::Client as OAuth2;
//!
//! let auth = OAuth2::new(
//!     Default::default(),
//!     "https://example.com/oauth2/auth",
//!     "https://example.com/oauth2/token",
//!     "CLIENT_ID",
//!     "CLIENT_SECRET",
//!     None
//! );
//! ```
//!
//! ## Constructing an authorization URI
//!
//! Direct the user to an authorization URI to have them authorize your application.
//!
//! ```
//! # use inth_oauth2::Client as OAuth2;
//! # let auth = OAuth2::google(Default::default(), "", "", None);
//! let auth_uri = auth.auth_uri(Some("scope"), Some("state")).unwrap();
//! ```
//!
//! ## Requesting an access token
//!
//! Using a code obtained from the redirect of the authorization URI, request an access token.
//!
//! ```no_run
//! # use inth_oauth2::Client as OAuth2;
//! # let auth = OAuth2::google(Default::default(), "", "", None);
//! # let code = String::new();
//! let token = auth.request_token(&code).unwrap();
//! println!("{}", token.access_token);
//! ```
//!
//! ## Refreshing an access token
//!
//! Refresh the access token when it has expired.
//!
//! ```no_run
//! # use inth_oauth2::Client as OAuth2;
//! # let auth = OAuth2::google(Default::default(), "", "", None);
//! # let mut token = auth.request_token("").unwrap();
//! if token.expired() {
//!     token = auth.refresh_token(&token, None).unwrap();
//! }
//! ```

extern crate chrono;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub use client::Client;
pub mod client;

pub use token::Token;
pub mod token;

pub use error::{Error, Result};
pub mod error;
