use actix_files::*;
use actix_web::*;
use std::path::PathBuf;

type IndexPageResult = Either<HttpResponse, NamedFile>;

pub async fn index(req: HttpRequest) -> Result<IndexPageResult> {
    let mut filename = req.match_info().query("filename");
    if filename.is_empty() {
        filename = "index.html"
    }
    let path: PathBuf = format!("pages/dist/{}", filename).parse()?;
    if !path.exists() {
        Ok(IndexPageResult::Left(
            HttpResponse::MovedPermanently()
                .append_header(("Location", "/"))
                .body(""),
        ))
    } else {
        Ok(IndexPageResult::Right(NamedFile::open(path)?))
    }
}
