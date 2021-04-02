mod convert_api;
mod index_page;

use actix_web::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Open http://localhost:8080/ in browser.");
    HttpServer::new(|| {
        App::new()
            .route("/api/convert/", web::post().to(convert_api::convert))
            .route("/{filename:.*}", web::get().to(index_page::index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
