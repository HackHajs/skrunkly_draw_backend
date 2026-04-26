use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    State as Bstate, api::Id, authentication::Authenticated, error::ErrorResponse,
    model::user::User,
};

/// Add the user to the database
///
/// # Errors
/// Will return an error if the post object is malformed or if the user is unauthenticated.
/// Might return an error if there's an issue communicating with the database.
pub async fn upsert(
    State(state): State<Arc<Bstate>>,
    Authenticated { sub, .. }: Authenticated,
    Json(user): Json<User>,
) -> Result<StatusCode, ErrorResponse> {
    user.upsert(&state.users, sub).await?;

    Ok(StatusCode::OK)
}

/// Fetch the user's public info
///
/// # Errors
/// Might return an error if there's an issue communicating with the database.
pub async fn get(
    State(state): State<Arc<Bstate>>,
    Query(id): Query<Id>,
) -> Result<Json<User>, ErrorResponse> {
    Ok(Json(User::get(&state.users, id.id).await?))
}

/// Remove the user from the database
///
/// # Errors
/// Will return an error if the user is unauthenticated.
/// Might return an error if there's an issue communicating with the database.
pub async fn delete(
    State(state): State<Arc<Bstate>>,
    Authenticated { sub, .. }: Authenticated,
) -> Result<Json<u64>, ErrorResponse> {
    Ok(Json(User::delete(&state.users, sub).await?))
}
