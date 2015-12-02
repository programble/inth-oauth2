//! # "It's not that hard" OAuth2 Client
//!
//! OAuth 2.0 really isn't that hard, you know?
//!
//! Implementation of [RFC6749](http://tools.ietf.org/html/rfc6749).
//!
//! `inth_oauth2` is on [Crates.io][crate] and [GitHub][github].
//!
//! [crate]: https://crates.io/crates/inth-oauth2
//! [github]: https://github.com/programble/inth-oauth2
//!
//! ## Providers
//!
//! `inth_oauth2` supports the following OAuth 2.0 providers:
//!
//! - `Google`
//! - `GitHub`
//! - `Imgur`
//!
//! Support for others can be added by implementing the `Provider` trait.
//!
//! ## Examples
//!
//! ### Creating a client
//!
//! ```
//! use inth_oauth2::{Client, Google};
//!
//! let client = Client::<Google>::new(
//!     Default::default(),
//!     "CLIENT_ID",
//!     "CLIENT_SECRET",
//!     Some("REDIRECT_URI")
//! );
//! ```
//!
//! ### Constructing an authorization URI
//!
//! ```
//! # use inth_oauth2::{Client, Google};
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! let auth_uri = client.auth_uri(Some("scope"), Some("state")).unwrap();
//! ```
//!
//! Direct the user to an authorization URI to have them authorize your application.
//!
//! ### Requesting an access token
//!
//! Request an access token using a code obtained from the redirect of the authorization URI.
//!
//! ```no_run
//! # use inth_oauth2::{Client, Google};
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let code = String::new();
//! let token_pair = client.request_token(&code).unwrap();
//! println!("{}", token_pair.access.token);
//! ```
//!
//! ### Refreshing an access token
//!
//! ```no_run
//! # use inth_oauth2::{Client, Google};
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let mut token_pair = client.request_token("").unwrap();
//! if token_pair.expired() {
//!     if let Some(refresh) = token_pair.refresh {
//!         token_pair = client.refresh_token(refresh, None).unwrap();
//!     }
//! }
//! ```
//!
//! ### Using bearer access tokens
//!
//! If the obtained token is of the `Bearer` type, a Hyper `Authorization` header can be created
//! from it.
//!
//! ```no_run
//! # extern crate hyper;
//! # extern crate inth_oauth2;
//! # fn main() {
//! # use inth_oauth2::{Client, Google};
//! # let client = Client::<Google>::new(Default::default(), "", "", None);
//! # let mut token_pair = client.request_token("").unwrap();
//! let client = hyper::Client::new();
//! let res = client.get("https://example.com/resource")
//!     .header(token_pair.to_bearer_header().unwrap())
//!     .send()
//!     .unwrap();
//! # }
//! ```
//!
//! ### Persisting tokens
//!
//! `TokenPair` implements `Encodable` and `Decodable` from `rustc_serialize`, so can be persisted
//! as JSON.
//!
//! ```
//! # extern crate inth_oauth2;
//! # extern crate rustc_serialize;
//! # extern crate chrono;
//! use inth_oauth2::{TokenPair, AccessTokenType, AccessToken, RefreshToken};
//! use rustc_serialize::json;
//! # use chrono::{UTC, Timelike};
//! # fn main() {
//! # let token_pair = TokenPair {
//! #     access: AccessToken {
//! #         token: String::from("AAAAAAAA"),
//! #         token_type: AccessTokenType::Bearer,
//! #         expires: Some(UTC::now().with_nanosecond(0).unwrap()),
//! #         scope: None,
//! #     },
//! #     refresh: Some(RefreshToken { token: String::from("BBBBBBBB") }),
//! # };
//!
//! let json = json::encode(&token_pair).unwrap();
//! let decoded: TokenPair = json::decode(&json).unwrap();
//! assert_eq!(token_pair, decoded);
//! # }
//! ```

extern crate chrono;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub use client::Client;
pub mod client;

pub use provider::{Provider, Google, GitHub, Imgur};
pub mod provider;

pub use token::{TokenPair, AccessTokenType, AccessToken, RefreshToken};
pub mod token;

pub use error::{Error, Result};
pub mod error;
