#![allow(warnings,dead_code, unused_variables)]

use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    Responder,
    HttpResponse
};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

use newsletter::run;
#[actix_web::main]
async fn main(){
    run().unwrap().await;
}
