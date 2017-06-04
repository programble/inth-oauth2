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
//! let client = Client::<Installed>::new(
//!     String::from("client_id"),
//!     String::from("client_secret"),
//!     Some(String::from("redirect_uri"))
//! );
//! ```
//!
//! ### Constructing an authorization URI
//!
//! ```
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # let client = Client::<Installed>::new(String::new(), String::new(), None);
//! let auth_uri = client.auth_uri(Some("scope"), Some("state")).unwrap();
//! println!("Authorize the application by clicking on the link: {}", auth_uri);
//! ```
//!
//! ### Requesting an access token
//!
//! ```no_run
//! use std::io;
//! use inth_oauth2::{Client, Token};
//! # use inth_oauth2::provider::google::Installed;
//! # let client = Client::<Installed>::new(String::new(), String::new(), None);
//!
//! let mut code = String::new();
//! io::stdin().read_line(&mut code).unwrap();
//!
//! let http_client = Default::default();
//! let token = client.request_token(&http_client, code.trim()).unwrap();
//! println!("{}", token.access_token());
//! ```
//!
//! ### Refreshing an access token
//!
//! ```no_run
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # let client = Client::<Installed>::new(String::new(), String::new(), None);
//! # let http_client = Default::default();
//! # let token = client.request_token(&http_client, "").unwrap();
//! let token = client.refresh_token(&http_client, token, None).unwrap();
//! ```
//!
//! ### Ensuring an access token is still valid
//!
//! ```no_run
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # let client = Client::<Installed>::new(String::new(), String::new(), None);
//! # let http_client = Default::default();
//! # let mut token = client.request_token(&http_client, "").unwrap();
//! // Refresh token only if it has expired.
//! token = client.ensure_token(&http_client, token).unwrap();
//! ```
//!
//! ### Using bearer access tokens
//!
//! Bearer tokens can be converted to Hyper headers.
//!
//! ```no_run
//! # extern crate hyper;
//! # extern crate hyper_native_tls;
//! # extern crate inth_oauth2;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! use hyper::header::Authorization;
//!
//! # fn main() {
//! let tls = hyper_native_tls::NativeTlsClient::new().unwrap();
//! let connector = hyper::net::HttpsConnector::new(tls);
//! let client = hyper::Client::with_connector(connector);
//! # let oauth_client = Client::<Installed>::new(String::new(), String::new(), None);
//! # let token = oauth_client.request_token(&client, "").unwrap();
//! let request = client.get("https://example.com/resource")
//!     .header(Into::<Authorization<_>>::into(&token));
//! # }
//! ```
//!
//! ### Persisting tokens
//!
//! All token types implement `Serialize` and `Deserialize` from `serde`.
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! extern crate serde_json;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::google::Installed;
//! # fn main() {
//! # let http_client = Default::default();
//! # let client = Client::<Installed>::new(String::new(), String::new(), None);
//! # let token = client.request_token(&http_client, "").unwrap();
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
    variant_size_differences
)]

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate hyper;
extern crate serde_json;
extern crate url;

pub mod token;
pub mod provider;
pub mod error;
pub mod client;

pub use token::{Token, Lifetime};
pub use client::{Client, ClientError};
