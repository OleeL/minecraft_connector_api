use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use serde::Deserialize;
use serde_json::json;
use std::error::Error as StdError;
use std::fmt;
use std::net::TcpStream;

mod address;
mod buffer;
mod messages;
mod server_status;

use address::Address;
use messages::send_message;

#[derive(Debug)]
struct ServiceError(Box<dyn StdError + Send + Sync>);

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl StdError for ServiceError {}
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(json!({ "error": self.to_string() }))
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
async fn status(query: web::Query<StatusQuery>) -> Result<HttpResponse, ServiceError> {
    // Heavy / blocking work should not happen on the Actix worker thread.
    // Off-load to a blocking task.
    let res = web::block(move || {
        let address = Address {
            url: query.url.clone(),
            port: query.port,
        };
        query_server(&address)
    })
    .await
    .map_err(|e| ServiceError(Box::new(e)))?;

    let result = res.map_err(ServiceError)?;

    Ok(HttpResponse::Ok().body(result))
}

fn query_server(address: &Address) -> Result<String, Box<dyn StdError + Send + Sync>> {
    let adr_str = format!("{}:{}", address.url, address.port);
    let stream = TcpStream::connect(&adr_str)?;

    let buff = send_message(&stream, address)?;
    Ok(String::from_utf8_lossy(&buff[..]).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://0.0.0.0:8080");
    HttpServer::new(|| App::new().service(status))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
