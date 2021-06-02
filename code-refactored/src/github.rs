use crate::{Repo, RepoClient, Result};
use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::header::USER_AGENT;

pub struct GithubClient ();

impl RepoClient for GithubClient {
    fn fetch_repos(&self, org: &str) -> Result<Vec<Repo>> {
        let url = format!("https://api.github.com/orgs/{}/repos", org);
        // let url = format!("http://localhost:10000/api/data/orgs/{}/repos", org);
        let client = Client::new();
        let response = client
            .get(url)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "rust-test")
            .query(&[("per_page", "100")])
            .send()?;
    
        let response = response.error_for_status()?;

        let repos: Vec<Repo> = response.json()?;
        Ok(repos)
    }
}