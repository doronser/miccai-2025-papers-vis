use actix_web::{test, web, App};
use api::routes::papers;
use infrastructure::config::Config;
use services::DataLoader;
use std::env;

/// Helper function to set up test environment configuration
pub fn setup_test_config() -> Config {
    // Set up test environment to use the source repository data
    env::set_var(
        "DATA_DIR",
        "/l2l/src/miccai-2025-papers-vis/backend/src/data",
    );
    env::set_var("SERVER_PORT", "8000");
    env::set_var("CORS_ORIGINS", "http://localhost:3000");

    Config::from_env().expect("Failed to load test config")
}

/// Create a test instance of the Actix app with all routes configured
pub fn create_test_app(
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let config = setup_test_config();
    let data_loader = web::Data::new(DataLoader::new(config));

    App::new()
        .app_data(data_loader.clone())
        .configure(papers::configure_routes)
}

/// Assert response status and deserialize JSON body
pub async fn assert_json_response<T: serde::de::DeserializeOwned>(
    resp: actix_web::dev::ServiceResponse,
    expected_status: u16,
) -> T {
    assert_eq!(
        resp.status().as_u16(),
        expected_status,
        "Expected status {}, got {}",
        expected_status,
        resp.status().as_u16()
    );

    test::read_body_json(resp).await
}

/// Assert response status only
pub fn assert_status(resp: &actix_web::dev::ServiceResponse, expected_status: u16) {
    assert_eq!(
        resp.status().as_u16(),
        expected_status,
        "Expected status {}, got {}",
        expected_status,
        resp.status().as_u16()
    );
}
