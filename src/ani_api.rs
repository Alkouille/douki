use oauth2::{basic::BasicClient, AuthUrl, ClientId, CsrfToken};
use url::Url;

pub const QL_URL: &str = "https://graphql.anilist.co";
pub const TOKEN_URL: &str = "https://anilist.co/api/v2/oauth/authorize";
pub const CLIENT_ID: &str = "5253";

#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct ListQueryArguments {
        pub name: String,
        pub list_type: MediaType,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", argument_struct = "ListQueryArguments")]
    pub struct ListQuery {
        #[arguments(r#type = args.list_type, user_name = args.name.clone())]
        pub media_list_collection: Option<MediaListCollection>,
    }

    /// represents 'MutateEntry''s arguments
    #[derive(cynic::FragmentArguments, Debug)]
    pub struct MutateEntryArguments {
        pub id: i32,
        pub media_id: i32,
        pub media_status: Option<MediaListStatus>,
        pub notes: Option<String>,
        pub priority: Option<i32>,
        pub progress: Option<i32>,
        pub progress_volumes: Option<i32>,
        pub started_at: Option<FuzzyDateInput>,
    }

    /// mutate an Anilist's list entry
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "MutateEntryArguments")]
    pub struct MutateEntry {
        #[arguments(id = args.id, media_id = args.media_id, status = args.media_status, notes = &args.notes, priority = args.priority, progress = args.progress, progress_volumes = args.progress_volumes, started_at = &args.started_at)]
        pub save_media_list_entry: Option<MediaList>,
    }

    /// represents 'MutateEntryStatus''s arguments
    #[derive(cynic::FragmentArguments, Debug)]
    pub struct MutateEntryStatusArguments {
        pub id: i32,
        pub media_id: i32,
        pub status: Option<MediaListStatus>,
    }

    /// mutate, for a given entry, a status on a Anilist entry
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "MutateEntryStatusArguments"
    )]
    pub struct MutateEntryStatus {
        #[arguments(id = args.id, media_id = args.media_id, status = args.status)]
        pub save_media_list_entry: Option<MediaList>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct MediaListCollection {
        pub lists: Option<Vec<Option<MediaListGroup>>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct MediaListGroup {
        pub entries: Option<Vec<Option<MediaList>>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct MediaList {
        pub media_id: i32,
        pub media: Option<Media>,
        pub notes: Option<String>,
        pub status: Option<MediaListStatus>,
        pub progress_volumes: Option<i32>,
        pub repeat: Option<i32>,
        pub priority: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Media {
        pub title: Option<MediaTitle>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct MediaTitle {
        pub romaji: Option<String>,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    pub enum MediaListStatus {
        Current,
        Planning,
        Completed,
        Dropped,
        Paused,
        Repeating,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    pub enum MediaType {
        Anime,
        Manga,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct FuzzyDateInput {
        pub year: Option<i32>,
        pub month: Option<i32>,
        pub day: Option<i32>,
    }
}

mod schema {
    cynic::use_schema!(r#"schema.graphql"#);
}

/*
TODO
Pretty sure this could be done at compile-time
 */
pub fn make_client_url() -> Result<(Url, CsrfToken), url::ParseError> {
    Ok(BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        None,
        AuthUrl::new(TOKEN_URL.to_string())?,
        None,
    )
    .authorize_url(CsrfToken::new_random)
    .use_implicit_flow()
    .url())
}
