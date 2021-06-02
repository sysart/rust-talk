use serde::Deserialize;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};

use std::collections::BinaryHeap;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize)]
struct Repo {
    stargazers_count: u32,
    name: String,
    language: Option<String>,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let language = match self.language.as_ref() {
            Some(s) => s,
            None => "ei määritelty"
        };
        write!(f, "Nimi: {}, kieli: {}, {}", self.name, language, self.stargazers_count)
    }
}

fn main() {
    let repos = fetch_repos();
    summarize(repos);
}

fn summarize(repos: Vec<Repo>) {
    let sorted_repos: BinaryHeap<_> = repos
        .into_iter()
        .filter(|r| r.stargazers_count > 0)
        .collect();
    for repo in sorted_repos {
        println!("{}", repo);
    }
}

fn fetch_repos() -> Vec<Repo> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/orgs/sysart/repos")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "rust-test")
        .query(&[("per_page", "100")])
        .send()
        .unwrap();

    let repos: Vec<Repo> = response.json().unwrap();
    repos
}