use axum::{
    routing::get,
    Router
    };

#[tokio::main]
async fn main(){
    std::fs::create_dir_all("data");
    // build our application with a single route
    let connection: Connection = sqlite::open("data/todo.db").unwrap();

    let app = Router::new().route("/", get(|| async {"Welcome to the backend CRUD API! Hey, Ipaye are you there "}));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
    .await
    .unwrap();

}
