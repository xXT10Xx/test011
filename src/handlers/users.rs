use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::Claims,
    database::Database,
    models::{UpdateUserRequest, User, UserResponse},
    utils::AppError,
};

pub async fn list_users(
    State(database): State<Database>,
    Extension(_claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(database.pool())
        .await?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(|user| user.into()).collect();

    Ok(Json(user_responses))
}

pub async fn get_user(
    State(database): State<Database>,
    Extension(_claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(database.pool())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(user.into()))
}

pub async fn update_user(
    State(database): State<Database>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    if claims.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    payload.validate()?;

    let mut query = "UPDATE users SET updated_at = NOW()".to_string();
    let mut params: Vec<String> = vec![];
    let mut param_count = 1;

    if let Some(email) = &payload.email {
        query.push_str(&format!(", email = ${}", param_count));
        params.push(email.clone());
        param_count += 1;
    }

    if let Some(username) = &payload.username {
        query.push_str(&format!(", username = ${}", param_count));
        params.push(username.clone());
        param_count += 1;
    }

    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut sqlx_query = sqlx::query_as::<_, User>(&query);
    
    for param in params {
        sqlx_query = sqlx_query.bind(param);
    }
    sqlx_query = sqlx_query.bind(user_id);

    let user = sqlx_query.fetch_one(database.pool()).await?;

    Ok(Json(user.into()))
}