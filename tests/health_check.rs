#![allow(warnings)]

use newsletter;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
	// Arrange
	spawn_app();
	// We need to bring in `reqwest`
	// to perform HTTP requests against our application.
	let client = reqwest::Client::new();

}


fn spawn_app(){
	let server = newsletter::run().expect("sdf");
	tokio::spawn(server);
}