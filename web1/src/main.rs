use axum::{
    extract::{Form, Path},
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, str::FromStr};
use tokio::runtime::Builder;
use serde_json::{from_str, to_string, Value};
use serde_json::map::Map;  


fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let app = Router::new()
        .route("/", get(root))
        .route("/json", post(create_user_json))
        .route("/url/:name/:age", post(create_user_get))
        .route("/form", post(create_user_form));
        //.layer(midware);

        let addr = SocketAddr::from_str("0.0.0.0:8882").unwrap();

        axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    });
}

async fn root() -> impl IntoResponse {
    "hello wolrd!"
}

async fn create_user_get(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let name = params.get("name").unwrap().clone();
    let age: i32 = params.get("age").unwrap().parse().unwrap();
    let user = User {
        id: 2000,
        name,
        age
    };

    (StatusCode::CREATED, Json(user))
}

async fn create_user_json (Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1000,
        name: payload.name,
        age: payload.age
    };

    (StatusCode::CREATED, Json(user))
}

async fn create_user_form(Form(input): Form<CreateUser>) -> impl IntoResponse {
    let user = User{
        id: 3000,
        name: input.name,
        age: input.age,
    };
    
    (StatusCode::CREATED, Json(user))
}


#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    age: i32,
}