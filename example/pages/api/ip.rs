use vercel_runtime::{run, Body, Error as VercelError, Request, Response, StatusCode};

pub type VercelResult<T> = Result<T, VercelError>;

async fn ip_handler(request: Request) -> VercelResult<Response<Body>> {
    Ok(ip_inner(request).await?)
}

async fn ip_inner(_request: Request) -> anyhow::Result<Response<Body>> {
    let resp = reqwest::get("https://httpbin.org/ip").await?.text().await?;
    println!("{resp:#?}");
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(resp.into())?)
}

#[tokio::main]
pub async fn main() -> VercelResult<()> {
    run(ip_handler).await
}
