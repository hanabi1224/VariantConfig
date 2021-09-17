use super::*;

async fn ip_handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    ip_inner(request)
        .await
        .map_err(|e| VercelError::new(&format!("{:?}", e)))
}

async fn ip_inner(_request: Request) -> anyhow::Result<impl IntoResponse> {
    let resp = reqwest::get("https://httpbin.org/ip").await?.text().await?;
    println!("{:#?}", resp);
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(resp)
        .expect("Internal Server Error");
    Ok(response)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(lambda_async!(ip_handler))
}
