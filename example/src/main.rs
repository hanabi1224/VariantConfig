mod convert_api;

use std::net::SocketAddr;

use axum::{routing::post, Router};
use axum_extra::routing::SpaRouter;

#[tokio::main]
async fn main() {
    async_main().await.unwrap()
}

async fn async_main() -> anyhow::Result<()> {
    let port = 8080;
    println!("Open http://localhost:{port}/ in browser.");

    let app = Router::new()
        .route("/api/convert/", post(convert_api::convert))
        .merge(SpaRouter::new("/", "pages/dist/"));
    serve(app, port).await
}

async fn serve(app: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
