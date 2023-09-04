use axum::{
	routing::{get, post},
	http::StatusCode,
	Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use mongodb::{
	bson::doc, 
	options::{ClientOptions, ServerApi, ServerApiVersion}, 
	Client
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
	// initialize tracing
	tracing_subscriber::fmt::init();

	// replace the placeholder with your Atlas connection string
	let uri = "mongodb://localhost:27017";
	let mut client_options = ClientOptions::parse(uri).await?;

	// set the server_api field of the client_options object to Stable API version 1
	let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
	client_options.server_api = Some(server_api);

	// create a new client and connect to the server
	let client = Client::with_options(client_options)?;

	// send a ping to confirm a successful connection
	client
		.database("pos_mobile")
		.run_command(doc! {"ping": 1}, None)
		.await?;

	println!("Pinged your deployment. You successfully connected to MongoDB!");

	// build our application with a route
	let app = Router::new()
		.route("/", get(root))
		.route("/users", post(create_user));

	// run our app with hyper, listening globally on port 3000
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

	tracing::debug!("listening on {}", addr);
	println!("listening on {}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();

	Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
	println!("root()");

	"Hello, World!"
}

async fn create_user(
	// this argument tells axum to parse the request body
	// as JSON into a `CreateUser` type
	Json(payload): Json<CreateUserReq>,
) -> (StatusCode, Json<User>) {
	println!("create_user()");

	// insert your application logic here
	let user = User {
		id: 1337,
		username: payload.username,
	};

	// this will be converted into a JSON response
	// with a status code of `201 Created`
	(StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUserReq {
	username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
	id: u64,
	username: String,
}