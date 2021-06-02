use std::env;
use std::fs;

pub fn get_repos(org: &str) -> String {
    let data = fs::read_to_string("src/repos.json")
        .expect("Something went wrong reading the file");
    data
}

// use reqwest::Error;
// // TODO try reqwest instead of hyper

// #[tokio::get_repos]
// pub async fn get_repos(org: &str) -> String {
//     let url = format!("https://api.github.com/orgs/{}/repos", org);
//     let response = reqwest::get(&url).await?;

//     let body = response.json().await?;
//     body
// }
