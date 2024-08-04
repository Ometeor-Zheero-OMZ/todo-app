// use crate::utils::SET_ENV_MSG;

use actix_cors::Cors;
use actix_web::http::header;
use dotenvy::dotenv;

pub fn cors() -> Cors {
    dotenv().ok();
    // let frontend_port = std::env::var("FRONTEND_PORT").expect(SET_ENV_MSG.get("NO_SET_ENV_VAR_FRONTEND_PORT").unwrap_or(&""));
    let frontend_port = "http://localhost:3000".to_string();

    let cors = Cors::default()
        .allowed_origin(&frontend_port)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
        .max_age(3600);

    cors
}