use std::convert::Infallible;

use axum::{
	body::{to_bytes, Body},
	http,
};


type Request = http::Request<Body>;
type Response = http::Response<Body>;


pub async fn reverse_proxy(req_in: Request) -> Result<Response, Infallible> {
	let uri = req_in.uri();
	let uri_fwd = http::Uri::builder()
		.scheme("https")
		.authority("koornbeurs.nl")
		.path_and_query(uri.path_and_query().unwrap().clone())
		.build()
		.expect("Failed to build URI");

	let (req_in_parts, req_in_body) = req_in.into_parts();
	let req_in_body_bytes = to_bytes(req_in_body, usize::MAX)
		.await
		.expect("Failed to read request body");

	// Replace URI hostname and HOST header
	let mut req_fwd_parts = req_in_parts.clone();
	req_fwd_parts.uri = uri_fwd;
	req_fwd_parts.headers.insert(http::header::HOST, "koornbeurs.nl".parse().unwrap());

	let req_fwd: ureq::Request = req_fwd_parts.into();

	let res = req_fwd.send_bytes(&req_in_body_bytes)
		.expect("Failed to send request");

	let res_koornbussified = koornbussify(res);
	Ok(res_koornbussified)
}


fn koornbussify(res: ureq::Response) -> Response {
	let status = res.status();
	let headers = res.headers_names().iter()
		.map(|name| {
			let value = res.header(name).unwrap();
			(name.clone(), value.to_string())
		})
		.collect::<Vec<_>>();

	let res_str = res.into_string().expect("Failed to read response body");
	let res_koornbussified = res_str
		.replace("koornbeurs", "koornbussy")
		.replace("Koornbeurs", "Koornbussy");

	let mut res_rebuilt = http::Response::builder()
		.status(status);

	for (name, value) in headers {
		res_rebuilt = res_rebuilt.header(name, value);
	}

	res_rebuilt
		.body(Body::from(res_koornbussified))
		.expect("Failed to rebuild response")
}
