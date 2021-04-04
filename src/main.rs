mod API;
use crate::API::queries::{MyQueryArguments};

use tokio;
use cynic::{http::SurfExt, QueryBuilder};
use cynic;



#[tokio::main]
async fn main() {
    let query = API::queries::MyQuery::build(MyQueryArguments {id: 701603});

    let response = surf::post("https://graphql.anilist.co/")
        .run_graphql(query)
        .await
        .unwrap()
        .data
        .unwrap();

    /* ugly testing */
    for elem in response.media_list_collection.unwrap().lists.unwrap() {
        for elem2 in elem.unwrap().entries.unwrap() {
            println!("{}", elem2.unwrap().media_id);
        }
    }
}