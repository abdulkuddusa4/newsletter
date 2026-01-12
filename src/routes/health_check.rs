use actix_web::{
    web,
    HttpRequest,
    Responder,
    HttpResponse
};
use actix_web::dev::Server;

pub async fn check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

