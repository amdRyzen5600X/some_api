use std::{sync::{Arc, RwLock}, usize};

use axum::{
    extract::{Query, State}, 
    http::StatusCode, 
    routing::get, 
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    username: String,
}

#[derive(Deserialize)]
struct UserId {
    id: u64
}

#[derive(Clone)]
#[derive(Serialize)]
struct User {
    id: u64,
    email: String,
    username: String,
}

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();
    let app = Router::new()
        .route("/users", get(get_user).post(create_user))
        .with_state(Arc::clone(&shared_state));
    let listner = tokio::net::TcpListener::bind("0.0.0.0:42069").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    users: Vec<User>
}

async fn create_user(Query(user): Query<CreateUser>, State(state): State<SharedState>) -> StatusCode{
    let user = User {
        id: state.read().unwrap().users.len() as u64,
        email: user.email,
        username: user.username,
    };
    state.write().unwrap().users.push(user);
    StatusCode::CREATED
}

async fn get_user(Query(id): Query<UserId>, State(state): State<SharedState>) -> Result<Json<User>, StatusCode>{
    if let Some(user) = state.read().unwrap().users.get(id.id as usize) {
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
