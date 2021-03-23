mod payloads;
use payloads::*;

use actix_files::*;
use actix_web::*;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let mut filename = req.match_info().query("filename");
    if filename.len() == 0 {
        filename = "index.html"
    }
    let mut path: PathBuf = format!("pages/dist/{}", filename).parse()?;
    if !path.exists() {
        path = "pages/dist/index.html".parse()?;
    }
    Ok(NamedFile::open(path)?)
}

#[post("/convert/")]
async fn convert(payload: web::Json<ConvertPayload>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("plain/text")
        .json(payload.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(convert)
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
