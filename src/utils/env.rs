
static HOST: &str = "HOST";
static PORT: &str = "PORT";
static DATABASE_URL: &str = "DATABASE_URL";
static GITHUB_CLIENT_ID: &str = "GITHUB_CLIENT_ID";
static GITHUB_CLIENT_SECRET: &str = "GITHUB_CLIENT_SECRET";
static GITHUB_AUTH_URI: &str = "GITHUB_AUTH_URI";
static GITHUB_TOKEN_URI: &str = "GITHUB_TOKEN_URI";
static AUTH_CALLBACK: &str = "AUTH_CALLBACK";

pub enum Env {
    Host,
    Port,
    DatabaseUri,
    GithubClientId,
    GithubClientSecret,
    GithubAuthUri,
    GithubTokenUri,
    AuthCallback,
}

impl Env {
    fn as_str(&self) -> &'static str {
        match self {
            Env::Host => HOST,
            Env::Port => PORT,
            Env::DatabaseUri => DATABASE_URL,
            Env::GithubClientId => GITHUB_CLIENT_ID,
            Env::GithubClientSecret => GITHUB_CLIENT_SECRET,
            Env::GithubAuthUri => GITHUB_AUTH_URI,
            Env::GithubTokenUri => GITHUB_TOKEN_URI,
            Env::AuthCallback => AUTH_CALLBACK,
        }
    }
}

pub fn get_env(env: Env) -> String {
    dotenv::var(env.as_str()).expect(format!("{} 必须在.env文件或环境变量中设置", env.as_str()).as_str())
}