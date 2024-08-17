use std::convert::Infallible;

use axum::{
	body::Body,
	http,
};


type Request = http::Request<Body>;
type Response = http::Response<Body>;


pub async fn reverse_proxy(mut req: Request) -> Result<Response, Infallible> {
	let client = ureq::Agent::new();

	let mut host = req.uri_mut().host().unwrap();
	host = "koornbeurs.nl";

	// TODO: convert http::Request to ureq::Request
	// https://docs.rs/ureq/latest/ureq/struct.Request.html#impl-From%3CBuilder%3E-for-Request
	todo!()
}
