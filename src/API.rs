#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct MyQueryArguments {
        pub id: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", argument_struct = "MyQueryArguments")]
    pub struct MyQuery {
        #[arguments(r#type = MediaType::Anime, user_id = args.id)]
        pub media_list_collection: Option<MediaListCollection>,
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
}

mod schema {
    cynic::use_schema!(r#"schema.graphql"#);
}
