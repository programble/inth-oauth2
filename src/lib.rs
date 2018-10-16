//! # "It's not that hard" OAuth 2.0
//!
//! OAuth 2.0 really isn't that hard, you know?
//!
//! Implementation of [RFC 6749](http://tools.ietf.org/html/rfc6749).
//!
//! `inth_oauth2` is on [Crates.io][crate] and [GitHub][github].
//!
//! [crate]: https://crates.io/crates/inth-oauth2
//! [github]: https://github.com/programble/inth-oauth2
//!
//! ## Providers
//!
//! Support for the following OAuth 2.0 providers is included:
//!
//! - Google
//!   - Web
//!   - Installed
//! - GitHub
//! - Imgur
//!
//! Support for other providers can be added by implementing the `Provider` trait.
//!
//! ## Token types
//!
//! The only supported token type is Bearer. Support for others can be added by implementing the
//! `Token` trait.
//!
//! ## Examples
//!
//! ### Creating a client
//!
//! ```
//! use inth_oauth2::Client;
//! use inth_oauth2::provider::google::Installed;
//!
//! let client = Client::new(
//!     Installed,
//!     String::from("client_id"),
//!     String::from("client_secret"),
//!     Some(String::from("redirect_uri")),
//! );
//! ```
//!
//! ### Constructing an authorization URI
//!
//! ```
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # let client = Client::new(Installed, String::new(), String::new(), None);
//! let auth_uri = client.auth_uri(Some("scope"), Some("state"));
//! println!("Authorize the application by clicking on the link: {}", auth_uri);
//! ```
//!
//! ### Requesting an access token
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! # extern crate reqwest;
//! use std::io;
//! use inth_oauth2::{Client, Token};
//! # use inth_oauth2::provider::google::Installed;
//! # fn main() {
//! # let client = Client::new(Installed, String::new(), String::new(), None);
//!
//! let mut code = String::new();
//! io::stdin().read_line(&mut code).unwrap();
//!
//! let http = reqwest::Client::new();
//! let token = client.request_token(&http, code.trim()).unwrap();
//! println!("{}", token.access_token());
//! # }
//! ```
//!
//! ### Refreshing an access token
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! # extern crate reqwest;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # fn main() {
//! # let client = Client::new(Installed, String::new(), String::new(), None);
//! # let http = reqwest::Client::new();
//! # let token = client.request_token(&http, "").unwrap();
//! let token = client.refresh_token(&http, token, None).unwrap();
//! # }
//! ```
//!
//! ### Ensuring an access token is still valid
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! # extern crate reqwest;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # fn main() {
//! # let client = Client::new(Installed, String::new(), String::new(), None);
//! # let http = reqwest::Client::new();
//! # let mut token = client.request_token(&http, "").unwrap();
//! // Refresh token only if it has expired.
//! token = client.ensure_token(&http, token).unwrap();
//! # }
//! ```
//!
//! ### Using bearer access tokens
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! # extern crate reqwest;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! use inth_oauth2::Token;
//!
//! # fn main() {
//! # let oauth_client = Client::new(Installed, String::new(), String::new(), None);
//! # let http = reqwest::Client::new();
//! # let token = oauth_client.request_token(&http, "").unwrap();
//! let request = http.get("https://example.com/resource")
//!     .bearer_auth(token.access_token())
//!     .build();
//! # }
//! ```
//!
//! ### Persisting tokens
//!
//! All token types implement `Serialize` and `Deserialize` from `serde`.
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! # extern crate reqwest;
//! extern crate serde_json;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # fn main() {
//! # let http = reqwest::Client::new();
//! # let client = Client::new(Installed, String::new(), String::new(), None);
//! # let token = client.request_token(&http, "").unwrap();
//! let json = serde_json::to_string(&token).unwrap();
//! # }
//! ```

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate url;

pub mod token;
pub mod provider;
pub mod error;
pub mod client;

pub use token::{Token, Lifetime};
pub use client::{Client, ClientError};
