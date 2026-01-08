#![allow(warnings)]

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


pub fn run(listener: TcpListener) -> std::io::Result<Server> {
	let server: Server = HttpServer::new(|| {
		App::new()
		.route("/health_check", web::get().to(health_check))
	})
	.listen(listener)?
	.run();
	Ok(server)
}