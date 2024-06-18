mod convert_api;

use std::net::SocketAddr;

use axum::{routing::post, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    async_main().await.unwrap()
}

async fn async_main() -> anyhow::Result<()> {
    let port = 8080;
    println!("Open http://localhost:{port}/ in browser.");

    let app = Router::new()
        .route("/api/convert/", post(convert_api::convert))
        .nest_service("/", ServeDir::new("pages/dist/"));
    serve(app, port).await
}

async fn serve(app: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listenr = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listenr, app).await?;
    Ok(())
}
