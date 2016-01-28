extern crate hyper;
extern crate inth_oauth2;

use inth_oauth2::Client;
use inth_oauth2::provider::*;

fn assert_get_uri_ok(uri: &str) {
    let client = hyper::Client::new();
    let response = client.get(uri).send().unwrap();
    assert_eq!(hyper::Ok, response.status);
}

#[test]
fn google_auth_uri_ok() {
    let client = Client::<Google>::new(
        Default::default(),
        "143225766783-ip2d9qv6sdr37276t77luk6f7bhd6bj5.apps.googleusercontent.com",
        "",
    ).redirect_uri("urn:ietf:wg:oauth:2.0:oob");
    let auth_uri = client.auth_uri(
        Some("https://www.googleapis.com/auth/userinfo.email"),
        Some("state")
    ).unwrap();
    assert_get_uri_ok(&auth_uri);
}

#[test]
fn github_auth_uri_ok() {
    let client = Client::<GitHub>::new(
        Default::default(),
        "01774654cd9a6051e478",
        ""
    ).redirect_uri("https://cmcenroe.me/oauth2-paste/");
    let auth_uri = client.auth_uri(Some("user"), Some("state")).unwrap();
    assert_get_uri_ok(&auth_uri);
}

#[test]
fn imgur_auth_uri_ok() {
    let client = Client::<Imgur>::new(
        Default::default(),
        "505c8ca804230e0",
        ""
    ).redirect_uri("https://cmcenroe.me/oauth2-paste/");
    let auth_uri = client.auth_uri(None, Some("state")).unwrap();
    assert_get_uri_ok(&auth_uri);
}
