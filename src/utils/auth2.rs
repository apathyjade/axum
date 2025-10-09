
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    RedirectUrl,
    TokenUrl,
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    Client,
    StandardTokenResponse,
    StandardErrorResponse,
    EmptyExtraTokenFields,
};
use std::env;

pub type GithubClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;
pub fn get_github_auth_client() -> GithubClient {
    let base_client = BasicClient::new(ClientId::new(env::var("GITHUB_CLIENT_ID").unwrap()))
        .set_client_secret(ClientSecret::new(env::var("GITHUB_CLIENT_SECRET").unwrap()))
        .set_auth_uri(AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap())
        .set_token_uri(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap())
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000/auth/callback".to_string()).unwrap());
    return base_client;
}