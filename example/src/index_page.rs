use actix_files::*;
use actix_web::*;
use std::path::PathBuf;

type IndexPageResult = Either<HttpResponse, NamedFile>;

pub async fn index(req: HttpRequest) -> Result<IndexPageResult> {
    let mut filename = req.match_info().query("filename");
    if filename.len() == 0 {
        filename = "index.html"
    }
    let path: PathBuf = format!("pages/dist/{}", filename).parse()?;
    if !path.exists() {
        Ok(IndexPageResult::A(
            HttpResponse::MovedPermanently()
                .header("Location", "/")
                .body(""),
        ))
    } else {
        Ok(IndexPageResult::B(NamedFile::open(path)?))
    }
}
