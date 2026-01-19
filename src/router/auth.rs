use axum::{
    Json,
    Router,
    extract::Query,
    // http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
};
use serde::Deserialize;
use std::time::Duration;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::{sync::RwLock, time::interval};
// use uuid::Uuid;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, basic::BasicClient,
};

use crate::{AppStateArc, model::api_response::ApiResponse, utils};

// 全局状态
type Sessions = Arc<RwLock<HashMap<String, Session>>>;
// type AuthStates = Arc<RwLock<HashMap<String, AuthState>>>;

#[derive(Clone)]
pub struct Session {
    // pub session_id: String,
    // pub user_id: String,
    // pub username: String,
    // pub roles: Vec<String>,
    // pub access_token: String,
    pub expires_at: std::time::Instant,
}

#[derive(Clone)]
// pub struct AuthState {
//     pub session_id: String,
//     pub csrf_token: String,
//     pub pkce_verifier: String,
//     pub created_at: std::time::Instant,
// }
#[derive(Debug, Deserialize)]
pub struct AuthCallbackParams {
    code: String,
    // state: String,
}

pub async fn auth_callback(Query(params): Query<AuthCallbackParams>) -> Json<ApiResponse<String>> {
    let req_client = reqwest::Client::new();
    let uri = utils::env::get_env(utils::env::Env::GithubAuthUri);
    let client_id = utils::env::get_env(utils::env::Env::GithubClientId);
    let client_secret = utils::env::get_env(utils::env::Env::GithubClientSecret);

    let result = req_client
        .post(uri)
        .header("Accept", "application/json")
        .json(&HashMap::from([
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", params.code),
        ]))
        .send()
        .await;
    match result {
        Ok(data) => {
            let val = data.text().await.unwrap();
            Json(ApiResponse::success(val))
        }
        Err(err) => Json(ApiResponse::error(err.to_string().as_str())),
    }
}

// 定期清理过期的会话
async fn clean_expired_sessions(sessions: Sessions) {
    let mut interval = interval(Duration::from_secs(60)); // 每分钟清理一次

    loop {
        interval.tick().await;

        let now = std::time::Instant::now();
        let mut sessions_to_remove = Vec::new();

        for (session_id, session) in sessions.read().await.iter() {
            if session.expires_at < now {
                sessions_to_remove.push(session_id.clone());
            }
        }

        for session_id in sessions_to_remove {
            sessions.write().await.remove(&session_id);
            println!("Removed expired session: {}", session_id);
        }
    }
}

async fn auth_github() -> impl IntoResponse {
    let client_id: ClientId = ClientId::new(utils::env::get_env(utils::env::Env::GithubClientId));
    let client_secret: ClientSecret =
        ClientSecret::new(utils::env::get_env(utils::env::Env::GithubClientSecret));
    let auth_url = AuthUrl::new(utils::env::get_env(utils::env::Env::GithubAuthUri)).unwrap();
    let token_url = TokenUrl::new(utils::env::get_env(utils::env::Env::GithubTokenUri)).unwrap();
    let redirect_url =
        RedirectUrl::new(utils::env::get_env(utils::env::Env::AuthCallback)).unwrap();

    let oauth_client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);
    let (_auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();
    Redirect::to(_auth_url.as_str())
}

pub fn router<'a>() -> Router<AppStateArc> {
    let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));

    // 启动定期清理过期的会话
    tokio::spawn(clean_expired_sessions(sessions.clone()));

    Router::new()
        .route("/github", get(auth_github))
        .route("/callback", get(auth_callback))
}
