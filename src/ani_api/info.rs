use lazy_static::lazy_static;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, CsrfToken};
use url::Url;

pub const QL_URL: &str = "https://graphql.anilist.co";
pub const TOKEN_URL: &str = "https://anilist.co/api/v2/oauth/authorize";
pub const CLIENT_ID: &str = "5253";
pub const SCHEMA_FILE: &str = "schema.graphql";

cynic::use_schema!(r#"schema.graphql"#);

lazy_static! {
    pub static ref CLIENT_URL: Result<(Url, CsrfToken), url::ParseError> = {
        Ok(BasicClient::new(
            ClientId::new(CLIENT_ID.to_string()),
            None,
            AuthUrl::new(TOKEN_URL.to_string())?,
            None,
        )
        .authorize_url(CsrfToken::new_random)
        .use_implicit_flow()
        .url())
    };
}
