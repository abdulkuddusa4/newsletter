use actix_web::HttpResponse;
use actix_web::{web, post};


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

