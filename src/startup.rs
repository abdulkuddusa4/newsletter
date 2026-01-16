use std::net::TcpListener;

use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    Responder,
    HttpResponse
};
use actix_web::middleware::Logger;
use actix_web::dev::Server;

use crate::routes::subscribe;
use crate::routes::check;

use env_logger::Env;


pub fn run(
	listener: TcpListener,
	db_pool: sqlx::PgPool
) -> std::io::Result<Server> {

	env_logger::Builder::from_env(
		Env::default().default_filter_or("info")
	).init();

	let db_pool = web::Data::new(db_pool);
	let server: Server = HttpServer::new(move || {
		App::new()
		.wrap(Logger::default())
		.route("/health_check", web::get().to(check))
		.service(subscribe)
		.app_data(db_pool.clone())
	})
	.listen(listener)?
	.run();
	Ok(server)
}