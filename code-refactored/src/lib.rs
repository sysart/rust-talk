// Implementation if the `git_client` library, which refactors the logic of our simple example

use std::collections::BinaryHeap;
use std::fmt;
use serde::Deserialize;

/// Struct fields correspond to field names in Github API
/// API docs at https://docs.github.com/en/rest/reference/repos
///
/// `stargazers_count` is the first field so the derived comparison traits
/// use it first, keeping the example simple
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

/// prints starred repos in decending order
pub fn summarize(repos: Vec<Repo>) {
    let sorted_repos: BinaryHeap<_> = repos
        .into_iter()
        .filter(|r| r.stargazers_count > 0)
        .collect();
    for repo in sorted_repos {
        println!("{}", repo);
    }
}

/// custom error type for `RepoClient`
/// `NotFound` is also an HTTP error, but we want to provide it separately
/// since it's the most common error
#[derive(Debug)]
pub enum CliError {
    Http(reqwest::Error),
    Json(String),
    NotFound,
}

/// Implementing the `From` trait allows us to return `reqwest::Error` from
/// method `fetch_repos` and the error gets automatically converted to our
/// `CliError`
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

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Http(ref err) => err.fmt(f),
            CliError::Json(ref err_text) => write!(f, "JSON parse error: {}", err_text),
            CliError::NotFound => write!(f, "Organization not found"),
        }
    }
}

pub type Result<T> = std::result::Result<T, CliError>;

/// A trait that all repo clients implement
/// Besides GithubClient, we could implement GitlabClient, BitbucketClient, etc.
pub trait RepoClient {
    fn fetch_repos(&self, org: &str) -> Result<Vec<Repo>>;
}

mod github;
pub use github::GithubClient;
