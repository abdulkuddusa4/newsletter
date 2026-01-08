#![allow(warnings)]

use newsletter;
use urlencoding as urlencode;
use std::net::TcpListener;

fn print_type_of<T>(obj: &T){
	println!("{:?}", std::any::type_name::<T>());
}
fn spawn_app()->String{
	let listener = TcpListener::bind("127.0.0.1:0")
		.expect("couldn't found any open port.");
	let port:u16 = listener.local_addr().unwrap().port();

	let server = newsletter::run(listener).expect("sdf");
	tokio::spawn(server);
	return format!("127.0.0.1:{}", port);
}


#[tokio::test]
async fn health_check_works() {
	// Arrange
	let addr: String = spawn_app();

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
	let addr: String = spawn_app();

	let client = reqwest::Client::new();

	let form_data = urlencode::encode("name=roni&email=abc@gmail.com")
		.to_string();
	let response = client
		.post(format!("http://{}/health_check",&addr))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(form_data)
		.send()
		.await
		.expect("Failed to execute request");


	print_type_of(&response);
	assert!(response.status().is_success());
	// assert_eq!(response.content_length(), Some(0));
}


#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
	// Arrange
	let app_address = spawn_app();
	let client = reqwest::Client::new();
	let test_cases = vec![
		("name=le%20guin", "missing the email"),
		("email=ursula_le_guin%40gmail.com", "missing the name"),
		("", "missing both name and email")
	];
	for (invalid_body, error_message) in test_cases {
		let response = client
			.post(&format!("{}/subscriptions", &app_address))
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
