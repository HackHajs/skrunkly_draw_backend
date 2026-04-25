use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    State as Bstate, authentication::Authenticated, error::ErrorResponse, model::user::User,
};

/// Add the user to the database
///
/// # Errors
/// Will return an error if the post object is malformed or if the user is unauthenticated.
/// Might return an error if there's an issue communicating with the database.
pub async fn new(
    State(state): State<Arc<Bstate>>,
    Authenticated { sub, .. }: Authenticated,
    Json(user): Json<User>,
) -> Result<StatusCode, ErrorResponse> {
    user.insert(&state.users, sub).await?;

    Ok(StatusCode::OK)
}
