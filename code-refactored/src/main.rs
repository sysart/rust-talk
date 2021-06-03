// this program provides the same functionality as our simple example
// but it's using the `git_client` library and provides better
// error handling

// crate name `git_client` is defined in `Cargo.toml`
use git_client::{RepoClient, GithubClient, CliError, summarize};

fn main() {
    // this is just showing how we can use GithubClient as a trait object
    // it would make more sense if we had several clients implementing
    // the `RepoClient` trait to choose from
    //
    // simple `let client = new GithubClient();` would work as well
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