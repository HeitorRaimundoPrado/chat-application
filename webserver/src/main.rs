use axum::extract::{FromRequestParts, Json, State};
use axum::http::request::Parts;
use axum::routing::post;
use axum::{async_trait, response::IntoResponse, routing::get, routing::get_service, Router};
use deadpool_diesel::{Manager, Pool};
use diesel::query_dsl::methods::SelectDsl;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use models::room::RoomModel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

use std::sync::Arc;

pub mod handlers;
pub mod models;

pub mod errors;
pub mod schema;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

struct ExtractUser(String);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_bearer) = parts.headers.get(AUTHORIZATION) {
            let token = auth_bearer
                .to_str()
                .unwrap()
                .trim_start_matches("Bearer ")
                .to_string();

            let mut validation = Validation::new(Algorithm::HS256);
            validation.validate_aud = false;
            let decoded = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(std::env::var("SUPABASE_SECRET").unwrap().as_ref()),
                &validation,
            )
            .unwrap();

            return Ok(ExtractUser(decoded.claims.sub));
        }

        Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}

async fn create_room(
    ExtractUser(_user_id): ExtractUser,
    State(app_state): State<models::app_state::AppState>,
    Json(body): Json<models::room::InsertRoomModel>,
) -> impl IntoResponse {
    let conn = app_state.pool.get().await.unwrap();
    let res = conn
        .interact(move |conn| {
            diesel::insert_into(schema::room::table)
                .values(body)
                .returning(models::room::RoomModel::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(crate::errors::adapt_error)
        .unwrap()
        .unwrap();

    Json(res).into_response()
}

async fn get_rooms(
    ExtractUser(_user_id): ExtractUser,
    State(app_state): State<models::app_state::AppState>,
) -> impl IntoResponse {
    let conn = app_state.pool.get().await.unwrap();
    let res: Vec<models::room::RoomModel> = conn
        .interact(move |conn| schema::room::table.load(conn))
        .await
        .map_err(crate::errors::adapt_error)
        .unwrap()
        .unwrap();

    Json(res).into_response()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url.to_string(), deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let service = ServeDir::new("src/static").fallback(ServeFile::new("src/static/index.html"));

    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(Mutex::new(tx));

    let mut senders: HashMap<usize, Arc<Mutex<broadcast::Sender<String>>>> = HashMap::new();
    senders.insert(0, tx);

    let app_state = models::app_state::AppState { senders, pool };

    let app = Router::new()
        .route("/api/hello/world", get(|| async { "Hello, World!" }))
        .route("/api/room/create", post(create_room))
        .route("/api/rooms/get", get(get_rooms))
        .route(
            "/api/message/listen",
            get(handlers::chat::broadcast_messages),
        )
        .route("/api/message/send", post(handlers::chat::send_message))
        .with_state(app_state)
        .fallback_service(get_service(service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
