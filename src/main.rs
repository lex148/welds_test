use crate::controllers::*;
use actix_web::{web::Data, App, HttpServer};
use models::dog::seed;
use std::env;
use std::net::SocketAddr;
mod controllers;
mod errors;
mod migrations;
mod models;
mod views;

use welds::connections::any::AnyClient;
pub(crate) type DbClient = actix_web::web::Data<AnyClient>;

const CS: &str =
    "server=127.0.0.1,11433;user id=sa;password=welds!123;TrustServerCertificate=true;";
// const CS: &str = "postgres://postgres:password@127.0.0.1:15432";
// const CS: &str = "mysql://root:welds!123@127.0.0.1:13306/mysql";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // default log level to info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    if let Err(err) = dotenvy::dotenv() {
        match err {
            dotenvy::Error::Io(_) => {}
            _ => log::warn!("DOTENV: {:?}", err),
        }
    }

    // read the environment variables to find what Interface to bind to
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let port = port.parse::<u16>().unwrap();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let ip: std::net::IpAddr = host.parse().unwrap();
    let bind_interface: SocketAddr = SocketAddr::new(ip, port);

    // Connect pick the one you want to test
    let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| CS.to_owned());
    let client = welds::connections::connect(&connection_string)
        .await
        .expect("Unable to connect to Database");

    migrations::up(&client).await.unwrap();

    let client = Data::new(client);
    seed(&client).await.unwrap();

    // boot up the server
    log::info!("Server Running at {}", bind_interface);

    if cfg!(debug_assertions) {
        log::info!("To start developing go to: http://localhost:{}", port);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(health_controller::index)
            .service(assets_controller::styles)
            .service(greetings_controller::index)
            .service(greetings_controller::reseed)
            .service(greetings_controller::ab)
            .service(greetings_controller::ab2)
    })
    .bind(bind_interface)?
    .run()
    .await
}
