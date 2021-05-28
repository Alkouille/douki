use crate::ani_api::info;
use cynic;

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub enum MediaType {
    Anime,
    Manga,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Query",
    argument_struct = "ListQueryArguments",
    schema_path = "schema.graphql",
    schema_module = "info"
)]
pub struct ListQuery {
    #[arguments(r#type = args.list_type, user_name = args.name.clone())]
    pub media_list_collection: Option<MediaListCollection>,
}

#[derive(cynic::FragmentArguments, Debug)]
pub struct ListQueryArguments {
    pub name: String,
    pub list_type: MediaType,
}

/// represents 'MutateEntry''s arguments
#[derive(cynic::FragmentArguments, Debug)]
pub struct MutateEntryArguments {
    pub id: Option<i32>,
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
#[cynic(
    graphql_type = "Mutation",
    argument_struct = "MutateEntryArguments",
    schema_path = "schema.graphql",
    schema_module = "info"
)]
pub struct MutateEntry {
    #[arguments(id = args.id, media_id = args.media_id, status = args.media_status, notes = &args.notes, priority = args.priority, progress = args.progress, progress_volumes = args.progress_volumes, started_at = &args.started_at)]
    pub save_media_list_entry: Option<MediaList>,
}

/// represents 'MutateEntryStatus''s arguments
#[derive(cynic::FragmentArguments, Debug)]

pub struct MutateEntryStatusArguments {
    pub id: Option<i32>,
    pub media_id: i32,
    pub status: Option<MediaListStatus>,
}

/// mutate, for a given entry, a status on a Anilist entry
#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Mutation",
    argument_struct = "MutateEntryStatusArguments",
    schema_path = "schema.graphql",
    schema_module = "info"
)]
pub struct MutateEntryStatus {
    #[arguments(id = args.id, media_id = args.media_id, status = args.status)]
    pub save_media_list_entry: Option<MediaList>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub struct MediaListCollection {
    pub lists: Option<Vec<Option<MediaListGroup>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub struct MediaListGroup {
    pub entries: Option<Vec<Option<MediaList>>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
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
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub struct Media {
    pub title: Option<MediaTitle>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub struct MediaTitle {
    pub romaji: Option<String>,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub enum MediaListStatus {
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "info")]
pub struct FuzzyDateInput {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

#[derive(cynic::FragmentArguments, Debug)]

pub struct TestArguments {
    pub media_id: i32,
    pub status: Option<MediaListStatus>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Mutation",
    argument_struct = "TestArguments",
    schema_path = "schema.graphql",
    schema_module = "info"
)]
pub struct Test {
    #[arguments(media_id = args.media_id, status = args.status)]
    pub save_media_list_entry: Option<MediaList>,
}
