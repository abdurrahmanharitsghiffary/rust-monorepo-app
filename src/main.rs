mod modules;
mod utils;

use std::env::set_var;
use actix_cors::Cors;
use actix_web::{
    http::header, middleware::Logger, web::{self, scope}, App, HttpServer
};
use modules::post::handler::post_config;
use sqlx::{Pool, Postgres};
use logger_libs;

pub type DbAppState = Pool<Postgres>;

pub struct AppState {
    db: DbAppState,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    env_logger::init();
    if std::env::var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=info");
    }

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL tidak ditemukan di environment");
    
    let port: u16 = dotenv::var("PORT")
        .expect("PORT tidak ditemukan di environment")
        .parse::<u16>()
        .expect("PORT harus berupa angka");

    let pool = create_db_pool(&database_url).await;

    logger_libs::Logger::info_logger("main","main",&format!("🚀 Server berjalan di port {:?}", port));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(configure_cors())
            .wrap(Logger::default())
            .service(
            scope("/api")
            .configure(post_config)
            )
            
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn create_db_pool(database_url: &str) -> Pool<Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .min_connections(5)
        .max_connections(50)
        .connect(database_url)
        .await
        .expect("Gagal membuat koneksi ke database")
}

fn configure_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}
