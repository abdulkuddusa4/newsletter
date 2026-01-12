#![allow(warnings)]
pub mod configuration;
pub mod routes;
pub mod startup;


use std::net::TcpListener;

use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    Responder,
    HttpResponse
};
use actix_web::dev::Server;


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}



