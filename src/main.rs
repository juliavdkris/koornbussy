use axum::{
    Router,
	routing::get,
	extract::Path
};
use tower_http::trace::TraceLayer;


const HOST: &str = "localhost";
const PORT: u16 = 8000;


#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let app = Router::new()
		.layer(TraceLayer::new_for_http())
		.route("/", get(hello))
		.route("/:name", get(hello_person));

	let listener = tokio::net::TcpListener::bind((HOST, PORT))
		.await
		.unwrap();
	tracing::debug!("Listening on http://{}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}


async fn hello() -> &'static str {
	"Hello world!"
}

async fn hello_person(Path(name): Path<String>) -> String {
	format!("Hello, {name}!")
}
