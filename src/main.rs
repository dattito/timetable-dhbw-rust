mod error;
mod ical;
mod routes;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    env_logger::init();

    if std::env::var("ICS_URL").is_err() {
        log::error!("environment variable 'ICS_URL' not set");
        std::process::exit(1);
    }

    let app = Router::new().route("/", get(routes::handler));

    log::info!("Starting server");
    axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
