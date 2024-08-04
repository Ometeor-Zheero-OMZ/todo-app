mod controllers;
mod models;
mod routes;
mod middlewares;
mod utils;

use crate::middlewares::cors;
use routes::router::config;
use utils::{DB_MSG, SET_ENV_MSG, SVR_MSG};

use actix_web::{middleware::Logger, App, HttpServer};
use sqlx::{PgPool, Error};
use dotenvy::dotenv;

pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let message: &str = SVR_MSG.get("SVR_BUILD_SUCCESS_MSG").unwrap_or(&"");
    println!("{}", message);

    // let database_url: String = std::env::var("DATABASE_URL").expect(SET_ENV_MSG.get("DATABASE_URL").unwrap_or(&""));
    let database_url = "postgres://root:root@db:5432/db".to_string();

    let db: PgPool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            let message: &str = DB_MSG.get("DB_CONNECTION_SUCCESS_MSG").unwrap_or(&"");
            println!("{}", message);
            pool
        }
        Err(err) => {
            let message: &str = DB_MSG.get("DB_CONNECTION_FAILURE_ERROR_MSG").unwrap_or(&"");
            println!("{}: {}", message, err);
            std::process::exit(1);
        }
    };

    // マイグレーションが必要な場合はここで実行します
    // sqlx::migrate!("./migrations").run(&db).await.unwrap();

    HttpServer::new(move || {
        let cors = cors();

        App::new()
            .app_data(actix_web::web::Data::new(AppState { db: db.clone() }))
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
