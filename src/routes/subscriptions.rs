use uuid::Uuid;
use chrono::{Utc};

use actix_web::HttpResponse;
use actix_web::{web, post};

use sqlx::{PgPool};


#[derive(serde::Deserialize)]
struct FormData{
	email: String,
	name: String
}


#[actix_web::post("/subscriptions")]
async fn subscribe(
	form: web::Form<FormData>,
	db_connection: web::Data<PgPool>
)
-> HttpResponse
{
	sqlx::query!(
		r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)	
		"#,
		Uuid::new_v4(),
		form.email,
		form.name,
		Utc::now()
	)
	.execute(db_connection.get_ref())
	.await;
    HttpResponse::Ok().finish()
}

