use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use mongodb::Collection;

use crate::{
    DATABASE_NAME, POST_COLLECTION_NAME, State as Bstate, authentication::Authenticated,
    error::ErrorResponse, model::post::Post,
};

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
    let posts: Collection<Post> = state
        .0
        .database(DATABASE_NAME)
        .collection(POST_COLLECTION_NAME);
    posts.insert_one(post).await.unwrap();

    Ok(StatusCode::OK)
}
