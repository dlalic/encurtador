use crate::common::{drop_database, start_backend};
use dotenv::dotenv;
use env_logger::Env;
use openapi::apis::configuration::Configuration;
use openapi::apis::default_api::{create_urls, search_urls};
use openapi::apis::Error;
use openapi::models::CreateUrl;
use tokio::sync::oneshot;
use url::Url;

mod common;

#[tokio::test]
async fn it_shortens_urls_and_counts_visits() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let (kill_sender_be, kill_receiver_be) = oneshot::channel::<()>();
    let (exit_status_sender_be, exit_status_receiver_be) = oneshot::channel::<()>();
    start_backend(exit_status_sender_be, kill_receiver_be).await;
    drop_database().await;

    let config = Configuration::default();

    let url = "https://foo.com/bar?baz=1234";
    let request = CreateUrl {
        url: url.to_string(),
    };
    let response = create_urls(&config, request).await.unwrap();
    let response_url = Url::parse(&response.shortened).unwrap();
    assert_eq!(response_url.domain().unwrap(), "tier.app");
    let shortened = response_url.path();

    match create_urls(
        &config,
        CreateUrl {
            url: url.to_string(),
        },
    )
    .await
    {
        Ok(_) => assert!(false, "Duplicate is allowed"),
        Err(err) => match err {
            Error::ResponseError(err) => assert_eq!(err.status, 400),
            _ => assert!(false, "Unexpected error"),
        },
    };

    let response = search_urls(&config, url).await.unwrap();
    let response_url = Url::parse(&response.shortened).unwrap();
    assert_eq!(response_url.domain().unwrap(), "tier.app");
    assert_eq!(response_url.path(), shortened);

    kill_sender_be.send(()).unwrap();
    exit_status_receiver_be.await.unwrap();
}
