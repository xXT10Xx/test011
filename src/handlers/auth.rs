use axum::{extract::State, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{create_jwt, hash_password, verify_password},
    config::Config,
    database::Database,
    models::{CreateUserRequest, LoginRequest, LoginResponse, User, UserResponse},
    utils::AppError,
};

pub async fn register(
    State(database): State<Database>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;

    let config = Config::from_env()?;
    let password_hash = hash_password(&payload.password, Some(config.bcrypt_cost))?;

    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&payload.username)
    .bind(&password_hash)
    .bind(now)
    .bind(now)
    .fetch_one(database.pool())
    .await?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(database): State<Database>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(database.pool())
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let config = Config::from_env()?;
    let token = create_jwt(user.id, &config.jwt_secret)?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}