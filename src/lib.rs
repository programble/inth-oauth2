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
//! use inth_oauth2::provider::Google;
//!
//! let client = Client::<Google>::new(
//!     Default::default(),
//!     "client_id",
//!     "client_secret",
//!     Some("redirect_uri")
//! );
//! ```
//!
//! ### Constructing an authorization URI
//!
//! ```
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::Google;
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! let auth_uri = client.auth_uri(Some("scope"), Some("state")).unwrap();
//! println!("Authorize the application by clicking on the link: {}", auth_uri);
//! ```
//!
//! ### Requesting an access token
//!
//! ```no_run
//! use std::io;
//! use inth_oauth2::{Client, Token};
//! # use inth_oauth2::provider::Google;
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//!
//! let mut code = String::new();
//! io::stdin().read_line(&mut code).unwrap();
//!
//! let token = client.request_token(code.trim()).unwrap();
//! println!("{}", token.access_token());
//! ```
//!
//! ### Refreshing an access token
//!
//! ```no_run
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::Google;
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let token = client.request_token("").unwrap();
//! let token = client.refresh_token(token, None).unwrap();
//! ```
//!
//! ### Ensuring an access token is still valid
//!
//! ```no_run
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::Google;
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let mut token = client.request_token("").unwrap();
//! // Refresh token only if it has expired.
//! token = client.ensure_token(token).unwrap();
//! ```
//!
//! ### Using bearer access tokens
//!
//! Bearer tokens can be converted to Hyper headers.
//!
//! ```no_run
//! # extern crate hyper;
//! # extern crate inth_oauth2;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::Google;
//! use hyper::header::Authorization;
//!
//! # fn main() {
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let token = client.request_token("").unwrap();
//! let client = hyper::Client::new();
//! let request = client.get("https://example.com/resource")
//!     .header(Into::<Authorization<_>>::into(&token));
//! # }
//! ```
//!
//! ### Persisting tokens
//!
//! All token types implement `Encodable` and `Decodable` from `rustc_serialize`.
//!
//! ```no_run
//! # extern crate inth_oauth2;
//! # extern crate rustc_serialize;
//! # use inth_oauth2::Client;
//! # use inth_oauth2::provider::Google;
//! use rustc_serialize::json;
//! # fn main() {
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let token = client.request_token("").unwrap();
//! let json = json::encode(&token).unwrap();
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
    unused_results,
    variant_size_differences
)]

extern crate chrono;
extern crate hyper;
extern crate rustc_serialize;
extern crate serde;
extern crate url;

pub use token::{Token, Lifetime};
pub use client::{Client, ClientError};

pub mod token;
pub mod provider;
pub mod error;
pub mod client;

#[cfg(test)]
extern crate serde_json;
