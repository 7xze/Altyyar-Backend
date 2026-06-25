use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tower_http::auth::ValidateRequest;
use tracing::instrument;

use crate::config::Config;
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar: String,
}

#[derive(Debug, Clone)]
pub struct AuthCache {
    cache: moka::sync::Cache<String, AuthUser>,
}

impl AuthCache {
    pub fn new() -> Self {
        Self {
            cache: moka::sync::Cache::builder()
                .max_capacity(10_000)
                .time_to_live(std::time::Duration::from_secs(300))
                .build(),
        }
    }

    pub fn get(&self, token: &str) -> Option<AuthUser> {
        self.cache.get(token)
    }

    pub fn set(&self, token: String, user: AuthUser) {
        self.cache.insert(token, user);
    }
}

pub struct AuthenticatedUser(pub AuthUser);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<Config>: axum::extract::FromRef<S>,
    AuthCache: axum::extract::FromRef<S>,
{
    type Rejection = Response;

    #[instrument(skip(parts, state))]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = Arc::<Config>::from_ref(state);
        let cache = AuthCache::from_ref(state);

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing Authorization header"}))).into_response()
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid Authorization format. Use: Bearer <token>"}))).into_response()
        })?;

        if token.is_empty() {
            return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Empty token"}))).into_response());
        }

        if let Some(cached) = cache.get(token) {
            return Ok(AuthenticatedUser(cached));
        }

        let user = verify_token_with_mastodon(config, token).await.map_err(|e| {
            (StatusCode::UNAUTHORIZED, Json(json!({"error": e.to_string()}))).into_response()
        })?;

        cache.set(token.to_string(), user.clone());
        Ok(AuthenticatedUser(user))
    }
}

#[instrument(skip(config))]
async fn verify_token_with_mastodon(config: &Config, token: &str) -> Result<AuthUser, AppError> {
    let url = format!("{}/api/v1/accounts/verify_credentials", config.mastodon_api_base);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| AppError::Unauthorized(format!("Auth service unreachable: {}", e)))?;

    if !resp.status().is_success() {
        return Err(AppError::Unauthorized("Invalid or expired token".into()));
    }

    #[derive(Deserialize)]
    struct MastodonAccount {
        id: String,
        username: String,
        #[serde(default)]
        display_name: String,
        #[serde(default)]
        avatar: String,
    }

    let account = resp
        .json::<MastodonAccount>()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse auth response: {}", e)))?;

    Ok(AuthUser {
        id: account.id,
        username: account.username,
        display_name: account.display_name,
        avatar: account.avatar,
    })
}
