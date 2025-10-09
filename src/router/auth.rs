use axum::{
    extract::{ConnectInfo, Query, State},
    // http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fmt::{format, Debug}};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
// use uuid::Uuid;

use oauth2::{
    basic::BasicClient,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl
};

use reqwest;

use crate::{model::api_response::ApiResponse, utils, AppStateArc};

// 全局状态
// type Sessions = Arc<RwLock<HashMap<String, Session>>>;
// type AuthStates = Arc<RwLock<HashMap<String, AuthState>>>;

#[derive(Clone)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub access_token: String,
    pub expires_at: std::time::Instant,
}

#[derive(Clone)]
pub struct AuthState {
    pub session_id: String,
    pub csrf_token: String,
    pub pkce_verifier: String,
    pub created_at: std::time::Instant,
}

#[derive(Debug, Deserialize)]
pub struct AuthCallbackParams {
    code: String,
    state: String,
}

pub async fn auth_callback(
    Query(params): Query<AuthCallbackParams>,
) -> Json<ApiResponse<String>> {
    let req_client = reqwest::Client::new();
    let result = req_client.post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&HashMap::from([
            ("client_id", env::var("GITHUB_CLIENT_ID").unwrap()),
            ("client_secret", env::var("GITHUB_CLIENT_SECRET").unwrap()),
            ("code", params.code),
        ]))
        .send()
        .await;
    match result {
        Ok(data) => {
            let val = data.text().await.unwrap();
            Json(ApiResponse::success(format!("{:?}", val)))
        },
        Err(err) => {
            return Json(ApiResponse::error(err.to_string().as_str()));
        }
    }

    
    // let http_client = reqwest::ClientBuilder::new()
    // // Following redirects opens the client up to SSRF vulnerabilities.
    //     .redirect(reqwest::redirect::Policy::none())
    //     .build()
    //     .expect("Client should build");
    
    
    // let token_result = utils::auth2::get_github_auth_client()
    //     .exchange_code(AuthorizationCode::new(params.code)).request_async(&http_client).await;

    // match token_result {
    //     Ok(token_result_obj) => {
    //         let token = token_result_obj.access_token().secret();
    //         Json(ApiResponse::success(token.to_uppercase()))
    //     },
    //     Err(err) => {
    //         return Json(ApiResponse::error(err.to_string().as_str()));
    //     }
    // }
    
}

// 定期清理过期的会话
// async fn clean_expired_sessions(sessions: Sessions) {
//     let mut interval = interval(Duration::from_secs(60)); // 每分钟清理一次
    
//     loop {
//         interval.tick().await;
        
//         let now = std::time::Instant::now();
//         let mut sessions_to_remove = Vec::new();
        
//         for (session_id, session) in sessions.read().await.iter() {
//             if session.expires_at < now {
//                 sessions_to_remove.push(session_id.clone());
//             }
//         }
        
//         for session_id in sessions_to_remove {
//             sessions.write().await.remove(&session_id);
//             println!("Removed expired session: {}", session_id);
//         }
//     }
// }

async fn auth_github() -> impl IntoResponse {
    let oauth_client = BasicClient::new(ClientId::new(env::var("GITHUB_CLIENT_ID").unwrap()))
    .set_client_secret(ClientSecret::new(env::var("GITHUB_CLIENT_SECRET").unwrap()))
    .set_auth_uri(AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap())
    .set_token_uri(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap())
    .set_redirect_uri(RedirectUrl::new("http://localhost:3000/auth/callback".to_string()).unwrap());
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();
    Redirect::to(auth_url.as_str())
}

pub fn router<'a>() -> Router<AppStateArc> {

    // 启动定期清理过期的会话
    // tokio::spawn(clean_expired_sessions(sessions.clone()));
    
    Router::new()
        .route("/github", get(auth_github))
        .route("/callback", get(auth_callback))
}
