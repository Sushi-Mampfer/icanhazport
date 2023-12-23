use axum::{
    extract::Path, routing::get, response::Json, Router, http::HeaderMap,
};

use tokio::net::TcpListener;
use port_scanner::scan_port_addr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:port", get(handle_request));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(Path(port): Path<u16>, headers: HeaderMap) -> Json<bool> {
    let ip: &str = headers.get("CF-Connecting-IP").unwrap().to_str().unwrap();
    return Json(scan_port_addr(format!("{}:{}",ip.to_string(), port.to_string())));
}