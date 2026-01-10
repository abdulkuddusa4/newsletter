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

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}


#[derive(serde::Deserialize)]
struct FormData{
	email: String,
	name: String
}


#[actix_web::post("/subscriptions")]
async fn subscribe(
	_form: web::Form<FormData>
)
-> HttpResponse
{
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> std::io::Result<Server> {
	let server: Server = HttpServer::new(|| {
		App::new()
		.route("/health_check", web::get().to(health_check))
		.service(subscribe)
	})
	.listen(listener)?
	.run();
	Ok(server)
}