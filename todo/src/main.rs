use actix_web::{HttpResponse, HttpServer, get, App, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello World!";
    Ok(HttpResponse::Ok().body(response_body))
}