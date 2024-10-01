use anyhow::Context;
use microblog::{auth_handler, get_handlers, post_handlers, upload_handler, ServerState};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, post},
    Router,
};

use tower_http::{cors::CorsLayer, services::ServeDir};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let secret_key = secrets
        .get("KEY")
        .context("Could not find KEY in Secrets")?;

    let state = ServerState { pool, secret_key };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin([
            "https://blog.nicolas-frey.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    // Set up routes for both serving the upload form and handling uploads
    let router = Router::new()
        .route("/post/:post_id/upload", post(upload_handler::upload)) // Handle file uploads on POST requests
        .route("/media", get(get_handlers::get_all_media))
        .route("/media/:media_id", get(get_handlers::get_media))
        .route(
            "/post/:post_id/media",
            get(get_handlers::get_all_media_by_post),
        )
        .route("/user/:author/post", post(post_handlers::create_post))
        .route("/login", post(auth_handler::login))
        .route("/register", post(auth_handler::register))
        .nest_service("/", ServeDir::new("public"))
        // TODO: configure middleware correctly
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_handler::guard,
        ))
        .with_state(state)
        .layer(cors);

    Ok(router.into())
}
