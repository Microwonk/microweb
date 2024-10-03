use core::str;

use anyhow::Context;
use microblog::{
    auth_handler, media_handler, post_handler, user_handler, ApiError, ApiResult, ServerState,
};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};

use tower_http::cors::CorsLayer;

// needs to be async for axum
async fn test() -> ApiResult<impl IntoResponse> {
    Ok(Html(
        str::from_utf8(include_bytes!("../public/index.html")).map_err(|e| {
            ApiError::werr(
                "Something went wrong.",
                StatusCode::INTERNAL_SERVER_ERROR,
                e,
            )
        })?,
    ))
}

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

    let router = Router::new()
        .merge(unauthenticated_routes(state.clone()))
        .merge(authenticated_routes(state.clone()));

    Ok(router.into())
}

fn unauthenticated_routes(state: ServerState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([
            "https://blog.nicolas-frey.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    Router::new()
        // get token
        .route("/login", post(auth_handler::login))
        // create user, get token
        .route("/register", post(auth_handler::register))
        // get media from db with media_id
        .route("/media/:media_id", get(media_handler::get_media))
        // get single post from slug
        .route("/post/:slug", get(post_handler::get_post))
        // get all posts from user with path
        .route("/user/:id/posts", get(post_handler::get_posts_by_user))
        // get all posts
        .route("/posts", get(post_handler::get_all_posts))
        // get upload
        .route("/upload/:id", get(media_handler::get_upload))
        // test route
        .route("/test", get(test))
        .with_state(state)
        .layer(cors)
}

fn authenticated_routes(state: ServerState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin([
            "https://blog.nicolas-frey.com"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
        ]);

    Router::new()
        // # ADMIN ROUTES
        // upload media (with file upload)
        .route("/post/:post_id/upload", post(media_handler::upload))
        // get every single media
        .route("/media", get(media_handler::get_all_media))
        // get every single media from a post
        .route(
            "/post/:post_id/media",
            get(media_handler::get_all_media_by_post),
        )
        // delete media
        .route("/upload/:id", delete(media_handler::delete_media))
        // create a post (if admin)
        .route("/user/post", post(post_handler::create_post))
        // once post is created, can be updated, fetched and deleted
        .route(
            "/user/post/:id",
            delete(post_handler::delete_post)
                .put(post_handler::update_post)
                .get(post_handler::get_post_by_id),
        )
        // get all posts from a user by their identity (token)
        .route("/user/posts", get(post_handler::get_posts_by_identity))
        // get all users
        .route("/users", get(user_handler::get_all_users))
        // get all users that are admins
        .route("/users/admins", get(user_handler::get_all_admin_users))
        // check if the user is an admin for frontend purposes
        .route("/user/admin", get(user_handler::is_admin))
        // get profile and update profile (user)
        .route(
            "/profile",
            get(user_handler::get_profile).put(user_handler::change_profile),
        )
        // auth middleware
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_handler::auth_guard,
        ))
        .with_state(state)
        .layer(cors)
}
