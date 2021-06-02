use git_client::{RepoClient, GithubClient, CliError, summarize};


fn main() {
    let client: Box<dyn RepoClient> = Box::new(GithubClient());
    match client.fetch_repos("sysart") {
        Ok(repos) => summarize(repos),
        Err(err) => print_error(err),
    };
}



fn print_error(err: CliError) {
    let message = match err {
        CliError::Http(reqwest_err) => format!("virhe http-yhteydessä - {}", reqwest_err.to_string()),
        CliError::Json(msg) => format!("palvelimen palauttama data ei vastaa määriteltyä: {}", msg),
        CliError::NotFound => From::from("haettua dataa ei löytynyt"),
    };
    eprintln!("Virhe: {}", message);
}