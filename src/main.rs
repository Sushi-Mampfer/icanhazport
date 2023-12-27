use std::net::SocketAddr;

use axum::{
    extract::Path, routing::get, response::Json, Router, http::HeaderMap,
};

use tokio::net::{
    TcpStream, TcpListener,
};
use tokio::time::{
    timeout, Duration,
};

//test

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:port", get(handle_request))
        .route("/", get(get_ip));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(Path(port): Path<u16>, headers: HeaderMap) -> Json<bool> {
    let ip: &str = headers.get("CF-Connecting-IP").unwrap().to_str().unwrap();
    return Json(scan_port(SocketAddr::new(ip.parse().unwrap(), port)).await);
}
async fn get_ip(headers: HeaderMap) -> Json<String> {
    let ip: String = headers.get("CF-Connecting-IP").unwrap().to_str().unwrap().parse().unwrap();
    return Json(ip);
}

async fn scan_port(addr: SocketAddr) -> bool {
    if let Ok(_) =timeout(Duration::from_secs(3), TcpStream::connect(addr)).await {
        true
    } else {
        false
    }
}

