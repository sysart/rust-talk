use std::collections::BinaryHeap;
use std::fmt;
use serde::Deserialize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct Repo {
    pub stargazers_count: u32,
    pub name: String,
    pub language: Option<String>,
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

pub fn summarize(repos: Vec<Repo>) {
    let sorted_repos: BinaryHeap<_> = repos
        .into_iter()
        .filter(|r| r.stargazers_count > 0)
        .collect();
    for repo in sorted_repos {
        println!("{}", repo);
    }
}

pub enum CliError {
    Http(reqwest::Error),
    Json(String),
    NotFound,
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError {
        eprintln!("{}", err);
        if err.is_status() {
            let status = err.status().unwrap();
            if status.is_client_error() && status.as_u16() == 404 {
                CliError::NotFound
            } else {
                CliError::Http(err)
            }
        } else if err.is_decode() {
            CliError::Json(err.to_string())
        } else {
            CliError::Http(err)
        }
    }
}

pub type Result<T> = std::result::Result<T, CliError>;

pub trait RepoClient {
    fn fetch_repos(&self, org: &str) -> Result<Vec<Repo>>;
}

mod github;
pub use github::GithubClient;
