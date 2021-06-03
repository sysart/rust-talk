// Simple script that reads public repositories shown at https://github.com/sysart
// and prints starred repositories in decending order

use serde::Deserialize;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};

use std::collections::BinaryHeap;
use std::fmt;

/// Struct fields correspond to field names in Github API
/// API docs at https://docs.github.com/en/rest/reference/repos
///
/// `stargazers_count` is the first field so the derived comparison traits
/// use it first, keeping the example simple
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
    // we get the repos in order by collecting them in a sorted data structure
    // notice that Rust iterators are lazy, so we always need to call collect or such
    let sorted_repos: BinaryHeap<_> = repos
        .into_iter()
        .filter(|r| r.stargazers_count > 0)
        .collect();
    for repo in sorted_repos {
        println!("{}", repo);
    }
}

/// Perform the API call
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