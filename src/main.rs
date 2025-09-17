use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, ResponseError};
use env_logger::Env;
use log::error;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

mod address;
mod buffer;
mod messages;
mod server_status;

use address::Address;
use messages::send_message_async;
use server_status::ServerStatus;

use actix_web::http::StatusCode as HttpStatus;

#[derive(Debug, Error)]
enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("connection to {0} timed out")]
    Timeout(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid server response: {0}")]
    Parse(#[from] serde_json::Error),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> HttpStatus {
        match self {
            ApiError::BadRequest(_) => HttpStatus::BAD_REQUEST,
            ApiError::Timeout(_) => HttpStatus::GATEWAY_TIMEOUT,
            ApiError::Io(_) | ApiError::Parse(_) => HttpStatus::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code(); // avoid naming this `status`
        error!("{} - {}", code, self);
        HttpResponse::build(code).json(json!({ "error": self.to_string() }))
    }
}

#[derive(Deserialize)]
struct StatusQuery {
    url: String,
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 {
    25565
}

#[get("/status")]
async fn status(query: web::Query<StatusQuery>) -> Result<web::Json<ServerStatus>, ApiError> {
    if query.url.trim().is_empty() {
        return Err(ApiError::BadRequest("url cannot be empty".into()));
    }

    let address = Address {
        url: query.url.clone(),
        port: query.port,
    };

    let result = query_server_async(&address).await?;
    let status: ServerStatus = serde_json::from_str(&result)?;
    Ok(web::Json(status))
}

async fn query_server_async(address: &Address) -> Result<String, ApiError> {
    let adr_str = format!("{}:{}", address.url, address.port);
    let mut stream: TcpStream = timeout(Duration::from_secs(5), TcpStream::connect(&adr_str))
        .await
        .map_err(|_| ApiError::Timeout(adr_str.clone()))??;

    let buff = send_message_async(&mut stream, address).await?;

    Ok(String::from_utf8_lossy(&buff[..]).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Default to info logs if RUST_LOG is not set (and include actix_web)
    let env = Env::default().default_filter_or("info,actix_web=info");
    env_logger::Builder::from_env(env)
        .format_timestamp_secs()
        .init();

    println!("Listening on http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(status).route(
            "/healthz",
            web::get().to(|| async { HttpResponse::Ok().body("ok") }),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
