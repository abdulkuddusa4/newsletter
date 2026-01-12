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
	db_pool: sqlx::PgPool
) -> std::io::Result<Server> {

	let db_pool = web::Data::new(db_pool);
	let server: Server = HttpServer::new(move || {
		App::new()
		.route("/health_check", web::get().to(check))
		.service(subscribe)
		.app_data(db_pool.clone())
	})
	.listen(listener)?
	.run();
	Ok(server)
}