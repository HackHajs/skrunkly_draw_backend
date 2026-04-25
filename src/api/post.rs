use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    State as Bstate, authentication::Authenticated, error::ErrorResponse, model::post::Post,
};

/// Get all posts from the database
///
/// # Errors
/// Will return an error if the post object is malformed or if the user is unauthenticated.
/// Might return an error if there's an issue communicating with the database.
pub async fn get_all(
    State(state): State<Arc<Bstate>>,
    Authenticated { .. }: Authenticated,
) -> Result<Json<Vec<Post>>, ErrorResponse> {
    Ok(Json(Post::get_all(&state.posts).await?))
}

/// Add the post to the database
///
/// # Errors
/// Will return an error if the post object is malformed or if the user is unauthenticated.
/// Might return an error if there's an issue communicating with the database.
pub async fn post(
    State(state): State<Arc<Bstate>>,
    Authenticated { sub, .. }: Authenticated,
    Json(post): Json<Post>,
) -> Result<StatusCode, ErrorResponse> {
    post.insert(&state.posts, sub).await?;

    Ok(StatusCode::OK)
}
