use std::convert::Infallible;

use axum::{
	body::{to_bytes, Body},
	http,
};

type Request = http::Request<Body>;
type Response = http::Response<Body>;


const PROXY_HOST: &str = "koornbeurs.nl";


pub async fn reverse_proxy(mut req: Request) -> Result<Response, Infallible> {
	let uri = req.uri_mut();
	*uri = http::Uri::builder()
		.scheme("https")
		.authority(PROXY_HOST)
		.path_and_query(uri.path_and_query().unwrap().clone())
		.build()
		.expect("Failed to build URI");

	let (mut req_fwd, req_body_bytes) = http_request_to_ureq(req).await;

	req_fwd = req_fwd.set("host", PROXY_HOST);

	let res = req_fwd.send_bytes(&req_body_bytes)
		.expect("Failed to send request");

	Ok(ureq_response_to_http(res).await)
}


async fn http_request_to_ureq(req: Request) -> (ureq::Request, Vec<u8>) {
	let (req_parts, req_body) = req.into_parts();
	let req_body_bytes = to_bytes(req_body, usize::MAX)
		.await
		.expect("Failed to read request body");

	let req: ureq::Request = req_parts.into();

	(req, req_body_bytes.into())
}


async fn ureq_response_to_http(res: ureq::Response) -> Response {
	let res_bytes: http::Response<Vec<u8>> = res.into();
	let (res_head, res_body) = res_bytes.into_parts();

	http::Response::from_parts(res_head, Body::from(res_body))
}
