extern crate chrono;
extern crate hyper;
extern crate inth_oauth2;
#[macro_use]
extern crate yup_hyper_mock;

use chrono::{UTC, Duration};
use inth_oauth2::{Client, ClientError, Token, Lifetime};
use inth_oauth2::error::OAuth2ErrorCode;

mod provider {
    use inth_oauth2::token::{Bearer, Static, Expiring, Refresh};
    use inth_oauth2::provider::Provider;

    #[derive(Default)]
    pub struct BearerStatic;
    impl Provider for BearerStatic {
        type Lifetime = Static;
        type Token = Bearer<Static>;
        fn auth_uri(&self) -> &'static str {
            "https://example.com/oauth/auth"
        }
        fn token_uri(&self) -> &'static str {
            "https://example.com/oauth/token"
        }
    }

    #[derive(Default)]
    pub struct BearerExpiring;
    impl Provider for BearerExpiring {
        type Lifetime = Expiring;
        type Token = Bearer<Expiring>;
        fn auth_uri(&self) -> &'static str {
            "https://example.com/oauth/auth"
        }
        fn token_uri(&self) -> &'static str {
            "https://example.com/oauth/token"
        }
    }

    #[derive(Default)]
    pub struct BearerRefresh;
    impl Provider for BearerRefresh {
        type Lifetime = Refresh;
        type Token = Bearer<Refresh>;
        fn auth_uri(&self) -> &'static str {
            "https://example.com/oauth/auth"
        }
        fn token_uri(&self) -> &'static str {
            "https://example.com/oauth/token"
        }
    }
}

mod connector {
    use hyper;

    mock_connector_in_order!(BearerStatic {
        include_str!("response/request_token_bearer_static.http")
    });

    mock_connector_in_order!(BearerExpiring {
        include_str!("response/request_token_bearer_expiring.http")
    });

    mock_connector_in_order!(BearerRefresh {
        include_str!("response/request_token_bearer_refresh.http")
        include_str!("response/refresh_token_bearer_full.http")
    });

    mock_connector_in_order!(BearerRefreshPartial {
        include_str!("response/request_token_bearer_refresh.http")
        include_str!("response/refresh_token_bearer_partial.http")
    });

    mock_connector_in_order!(InvalidRequest {
        include_str!("response/invalid_request.http")
    });

    mock_connector_in_order!(RefreshInvalidRequest {
        include_str!("response/request_token_bearer_refresh.http")
        include_str!("response/invalid_request.http")
    });
}

macro_rules! mock_client {
    ($p:ty, $c:ty) => {
        (Client::<$p>::new(
            String::from("client_id"),
            String::from("client_secret"),
            None
        ),
        hyper::Client::with_connector(<$c>::default()))
    }
}


#[test]
fn request_token_bearer_static_success() {
    let (client, http_client) = mock_client!(provider::BearerStatic, connector::BearerStatic);
    let token = client.request_token(&http_client, "code").unwrap();
    assert_eq!("aaaaaaaa", token.access_token());
    assert_eq!(Some("example"), token.scope());
}

#[test]
fn request_token_bearer_expiring_success() {
    let (client, http_client) = mock_client!(provider::BearerExpiring, connector::BearerExpiring);
    let token = client.request_token(&http_client, "code").unwrap();
    assert_eq!("aaaaaaaa", token.access_token());
    assert_eq!(Some("example"), token.scope());
    assert_eq!(false, token.lifetime().expired());
    assert!(token.lifetime().expires() > &UTC::now());
    assert!(token.lifetime().expires() <= &(UTC::now() + Duration::seconds(3600)));
}

#[test]
fn request_token_bearer_refresh_success() {
    let (client, http_client) = mock_client!(provider::BearerRefresh, connector::BearerRefresh);
    let token = client.request_token(&http_client, "code").unwrap();
    assert_eq!("aaaaaaaa", token.access_token());
    assert_eq!(Some("example"), token.scope());
    assert_eq!("bbbbbbbb", token.lifetime().refresh_token());
    assert_eq!(false, token.lifetime().expired());
    assert!(token.lifetime().expires() > &UTC::now());
    assert!(token.lifetime().expires() <= &(UTC::now() + Duration::seconds(3600)));
}

#[test]
fn refresh_token_bearer_full() {
    let (client, http_client) = mock_client!(provider::BearerRefresh, connector::BearerRefresh);
    let token = client.request_token(&http_client, "code").unwrap();
    let token = client.refresh_token(&http_client, token, None).unwrap();
    assert_eq!("cccccccc", token.access_token());
    assert_eq!(Some("example"), token.scope());
    assert_eq!("dddddddd", token.lifetime().refresh_token());
    assert_eq!(false, token.lifetime().expired());
    assert!(token.lifetime().expires() > &UTC::now());
    assert!(token.lifetime().expires() <= &(UTC::now() + Duration::seconds(3600)));
}

#[test]
fn refresh_token_bearer_partial() {
    let (client, http_client) = mock_client!(provider::BearerRefresh, connector::BearerRefreshPartial);
    let token = client.request_token(&http_client, "code").unwrap();
    let token = client.refresh_token(&http_client, token, None).unwrap();
    assert_eq!("cccccccc", token.access_token());
    assert_eq!(Some("example"), token.scope());
    assert_eq!("bbbbbbbb", token.lifetime().refresh_token());
    assert_eq!(false, token.lifetime().expired());
    assert!(token.lifetime().expires() > &UTC::now());
    assert!(token.lifetime().expires() <= &(UTC::now() + Duration::seconds(3600)));
}

#[test]
fn request_token_bearer_static_wrong_lifetime() {
    let (client, http_client) = mock_client!(provider::BearerStatic, connector::BearerRefresh);
    let err = client.request_token(&http_client, "code").unwrap_err();
    assert!(match err { ClientError::Parse(..) => true, _ => false });
}

#[test]
fn request_token_bearer_expiring_wrong_lifetime() {
    let (client, http_client) = mock_client!(provider::BearerExpiring, connector::BearerRefresh);
    let err = client.request_token(&http_client, "code").unwrap_err();
    assert!(match err { ClientError::Parse(..) => true, _ => false });
}

#[test]
fn request_token_bearer_refresh_wrong_lifetime() {
    let (client, http_client) = mock_client!(provider::BearerRefresh, connector::BearerStatic);
    let err = client.request_token(&http_client, "code").unwrap_err();
    assert!(match err { ClientError::Parse(..) => true, _ => false });
}

#[test]
fn request_token_invalid_request() {
    let (client, http_client) = mock_client!(provider::BearerStatic, connector::InvalidRequest);
    let err = client.request_token(&http_client, "code").unwrap_err();
    assert!(match err {
        ClientError::OAuth2(err) => {
            assert_eq!(OAuth2ErrorCode::InvalidRequest, err.code);
            assert_eq!("example", err.description.unwrap());
            assert_eq!("https://example.com/error", err.uri.unwrap());
            true
        },
        _ => false,
    });
}

#[test]
fn refresh_token_invalid_request() {
    let (client, http_client) = mock_client!(provider::BearerRefresh, connector::RefreshInvalidRequest);
    let token = client.request_token(&http_client, "code").unwrap();
    let err = client.refresh_token(&http_client, token, None).unwrap_err();
    assert!(match err {
        ClientError::OAuth2(err) => {
            assert_eq!(OAuth2ErrorCode::InvalidRequest, err.code);
            assert_eq!("example", err.description.unwrap());
            assert_eq!("https://example.com/error", err.uri.unwrap());
            true
        },
        _ => false,
    });
}
