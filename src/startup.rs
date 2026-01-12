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

use crate::routes::subscribe;
use crate::routes::check;


pub fn run(
	listener: TcpListener,
	db_connection: sqlx::PgConnection
) -> std::io::Result<Server> {
	let server: Server = HttpServer::new(|| {
		App::new()
		.route("/health_check", web::get().to(check))
		.service(subscribe)
		.app_data(db_connection.clone())
	})
	.listen(listener)?
	.run();
	Ok(server)
}