#![allow(warnings)]

use newsletter::startup::run as newsletter_runner;
use urlencoding as urlencode;
use std::net::TcpListener;

use newsletter::configuration::get_configuration;

use sqlx::{PgConnection, PgPool, Connection};
// use sqlx_postgres::PgConnection;

fn print_type_of<T>(obj: &T){
	println!("{:?}", std::any::type_name::<T>());
}

pub struct TestApp{
	pub address: String,
	pub db_pool: PgPool
}
async fn spawn_app()->TestApp{
	let listener = TcpListener::bind("127.0.0.1:0")
		.expect("couldn't found any open port.");
	let port:u16 = listener.local_addr().unwrap().port();
	let address = format!("127.0.0.1:{}", port);

	let configuration = 
		get_configuration()
		.expect("Failed to laod config");

	let db_pool = PgPool::connect(
		&configuration.database.connection_string()
	)
	.await
	.expect("Failed to connect to Postgres.");

	let server = newsletter_runner(listener, db_pool.clone()).expect("sdf");
	tokio::spawn(server);
	return TestApp{address, db_pool};
}


#[tokio::test]
async fn health_check_works() {
	// Arrange
	let TestApp{address: addr, db_pool} = spawn_app().await;

	let client = reqwest::Client::new();
	let response = client
		.get(format!("http://{}/health_check",&addr))
		.send()
		.await
		.expect("Failed to execute request");

	assert!(response.status().is_success());
	assert_eq!(response.content_length(), Some(0));
}


#[tokio::test]
async fn subscribe_returns_a_200_valid_form_data() {
	let TestApp{address: addr, db_pool} = spawn_app().await;

	let configuration = get_configuration()
		.expect("failed to load config");

	let db_connection_string = configuration.database.connection_string();

	let mut db_connection = PgConnection::connect(&db_connection_string)
		.await
		.expect("Faield to connect to postgres");
	
	let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
		.fetch_one(&mut db_connection)
		.await
		.expect("Failed to fetch saved subscription.");

	assert_eq!(saved.email, "ursula_le_guin@gmail.com");
	assert_eq!(saved.name, "le guin");
	let client = reqwest::Client::new();

	let form_data = urlencode::encode("name=roni&email=abc@gmail.com")
		.to_string();
	let response = client
		.post(format!("http://{}/subscriptions",&addr))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.form(&[
			("name", "roni"),
			("email", "asdf")
		])
		.send()
		.await
		.expect("Failed to execute request");

	dbg!(response.text().await);
	// print_type_of(&response);
	// dbg!(response.status());
	// assert!(response.status().is_success());
	// assert_eq!(response.content_length(), Some(0));
}


// #[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
	// Arrange
	let TestApp{address: addr, db_pool} =
		spawn_app().await;

	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=ursula_le_guin%40gmail.com", "missing the name"),
		("", "missing both name and email")
	];
	for (invalid_body, error_message) in test_cases {
		let response = client
			.post(&format!("{}/subscriptions", &addr))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(invalid_body)
			.send()
			.await
			.expect("Failed to execute request.");
		// Assert
		assert_eq!(
			400,
			response.status().as_u16(),
			"The API did not fail with 400 Bad Request when the payload was {}.",
			error_message
		);
	}
}
