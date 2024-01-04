use axum::http::StatusCode;

pub async fn hello() -> Result<String, StatusCode> {
    Ok("\n<h1>Hello world!</h1>\n\n".to_string())
}