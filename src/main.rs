mod ani_api;

use crate::ani_api::queries::{
    ListQueryArguments, MediaType::Manga
};

use cynic;
use cynic::{http::ReqwestExt, http::SurfExt, QueryBuilder, MutationBuilder};
use std::io;

#[tokio::main]
async fn main() {
    let mut ani_username = String::new();

    println!("Please specifiy your anilist username");
    io::stdin()
        .read_line(&mut ani_username)
        .expect("Failed to read line");

    let query = ani_api::queries::ListQuery::build(ListQueryArguments {
        name: String::from(ani_username.trim()),
        list_type: Manga,
    });

    let response = surf::post(ani_api::info::QL_URL)
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

    let (auth_url, _) = ani_api::info::CLIENT_URL.as_ref().unwrap();

    println!(
        "Please browse to: {} and retrieve back the token!",
        auth_url
    );

    let mut token = String::new();

    io::stdin()
        .read_line(&mut token)
        .expect("Failed to read line");



    let query = ani_api::queries::Test::build(ani_api::queries::TestArguments {
        media_id: 30642,
        status: Some(ani_api::queries::MediaListStatus::Planning),
    });


    let authorization = format!("{}{}", "Bearer ", token.trim());

    let client = reqwest::Client::new();
    let response = client
        .post(ani_api::info::QL_URL)
        .header("Authorization", authorization)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .run_graphql(query)
        .await
        .unwrap()
        .data
        .unwrap();

    match response.save_media_list_entry {
        Some(x) => println!("{}", x.media_id),
        None => (),
    };
}
