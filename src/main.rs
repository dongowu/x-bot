use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 定义路由
    let app = Router::new()
        .route("/", get(root))
        .route("/api", post(handle_request));

    // 服务地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server is running at http://{}", addr);

    // 绑定并启动服务
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 根路由处理函数
async fn root() -> &'static str {
    "Welcome to Rust API!"
}

// API 路由处理函数
async fn handle_request(Json(payload): Json<Payload>) -> Json<Response> {
    let response = Response {
        message: format!("Hello, {}!", payload.name),
    };
    Json(response)
}

// 定义请求和响应的结构体
#[derive(Deserialize)]
struct Payload {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}
