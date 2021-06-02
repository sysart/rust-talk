# Sysart Tech Talks

Esitellään kiinnostavia aiheita tai pohditaan yhdessä ratkaisuja teknisiin ongelmiin

![Tekninen osaaminen](src/osaaminen.png)

---

# Sysart Tech Talk: Rust

* Mikä Rust?
* Yksinkertainen esimerkkiohjelma
* Refaktoroidaan esimerkki paremmaksi
* Mietteitä Rustista ja kielten oppimisesta
* Keskustelua

---

# Lähtökohdat

- Sysartin projekteissa käytössä perinteisiä oliokieliä (Java, C#) ja dynaamisia kieliä (TypeScript/JavaScript)
- pidetään mielessä laadukas ohjelmistokehitys
- kielissä ja paradigmoissa ei yleensä ole oikeaa tai väärää, vaan pääasia että hallitsee käytettävän tavan
- hyvä kieli auttaa ohjelmoijaa pääsemään hyvään lopputulokseen

---

# Rust 👋

- alkuperäinen idea: C++:n muistiturvallinen korvaaja
  - aivan alunperin suunniteltu parantamaan Firefoxin muistiturvallisuutta ja nopeutta
- kehitys alkanut vuonna 2006 Mozillan työntekijän harrastusprojektina, julkaistu 2010
- kielen rakenteet tähtäävät tehokkuuteen eikä helppokäyttöisyyteen
- käyttö yritysmaailmassa lisääntynyt

---

# Featureita

- vahva staattinen tyypitys hyvällä tyyppienpäättelyllä
- olio- ja funktio-ohjelmoinnin rakenteet
- ei automaattista roskienkeruuta (GC)
- ei `null`-arvoa vaan `Option<T>`
- ei poikkeuksia vaan `Result<T, E>`
- pattern matching
- ilmaisuvoimaiset makrot

---

# Hello world

```text
$ cargo new hello
     Created binary (application) `hello` package
$ cd hello
$ cargo run
   Compiling hello v0.1.0 (/Users/matti/code/rust/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.51s
     Running `target/debug/hello`
Hello, world!
```

---

# Esimerkkiohjelma

Listaa tähtiä saaneista Sysartin julkisista Github-repoista laskevassa järjestyksessä:

- nimi
- kieli
- tähtien määrä

Tehdään aluksi mahdollisimman suoraviivainen toteutus.

---

```rust
// main.rs
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
```

---

```rust
use serde::Deserialize;

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
```

---

```rust
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};

fn fetch_repos() -> Vec<Repo> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/api/data/orgs/sysart/repos")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "rust-test")
        .send()
        .unwrap();

    let repos: Vec<Repo> = response.json().unwrap();
    repos
}
```

---

```text
$ cargo run
Nimi: dtangler, kieli: Java, 17
Nimi: ansible-jenkins-docker, kieli: ei määritelty, 11
Nimi: webcam-directive, kieli: JavaScript, 2
Nimi: Android-Sleep-Log, kieli: Java, 1
Nimi: livehelperchat, kieli: PHP, 1
Nimi: scala-game-of-life, kieli: Scala, 1
Nimi: dashboard, kieli: JavaScript, 1
```

---

# Virheenkäsittely

- Rustissa ei poikkeuksia, `unwrap`-kutsun sijaan palautetaan virhe
- ei pitkiä stacktraceja (ellei erikseen laita niitä päälle)

---
# Millaisia virheitä voi tulla

- palvelinta ei löydy (yhteys DNS:ään poikki tai nimi määritelty väärin)
- yhteyden muodostus ei onnistu (palvelin ei kuuntele/palomuuri blokkaa)
- yhteys muodostuu mutta protokolla väärä
- palvelin ei palaa määräajassa (timeout)
- http-tason virhe
- virhe JSON-parsinnassa

---

# Tehdään seuraava mallinnus

```rust
pub enum CliError {
    Http(reqwest::Error),
    Json(String),
    NotFound,
}

type Result<T> = std::result::Result<T, CliError>;

pub trait RepoClient {
    fn fetch_repos(&self, org: &str) -> Result<Vec<Repo>>;
}
```

---

```rust
pub struct GithubClient ();

impl RepoClient for GithubClient {
    fn fetch_repos(&self, org: &str) -> Result<Vec<Repo>> {
        let url = format!("https://api.github.com/api/data/orgs/{}/repos", org);
        let client = Client::new();
        let response = client
            .get(url)
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, "rust-test")
            .send()?;
    
        let response = response.error_for_status()?;

        let repos: Vec<Repo> = response.json()?;
        Ok(repos)
    }
}
```

---

```rust
impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError {
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
```

---

```rust
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
```

---

- Tyypille `CliError` pitäisi toteuttaa vielä traitit `Error` ja `fmt::Display`, jotta tyyppi olisi yleiskäyttöinen

---
# Esityksessä ei käyty läpi

- muistinhallinta (ownership, borrow, move)
- async Rust
  - http-clientia käytetään yleensä async-koodista ettei blokata säiettä kun odotellaan IO:ta

---

Esimerkki: kielen käsitteet ymmärrettävä koodatessa:

```rust
fn main() {
    let repos = fetch_repos();
    summarize(repos);
    println!("{}", repos[0]);
}
```

```text
error[E0382]: borrow of moved value: `repos`
  --> src/main.rs:28:20
   |
26 |     let repos = fetch_repos();
   |         ----- move occurs because `repos` has type `Vec<Repo>`, which does not implement the `Copy` trait
27 |     summarize(repos);
   |               ----- value moved here
28 |     println!("{}", repos[0]);
   |                    ^^^^^ value borrowed here after move
```



---

# Kokemuksia

- kääntäjä on todella tiukka: kielen lukeminen on helppoa mutta koodatessa hallittava kielen peruskäsitteet hyvin
  - virheilmoitukset hyvin kuvaavia ja ohjaavat yleensä oikealle polulle
  - kääntäminen hidasta, ajaminen nopeaa
- jättää ohjelmoijan vastuulle monia asioita, joista ei normaalisti tarvitse huolehtia (low level)
- käytännön tekeminen edellyttää nopeasti advanced featureiden hallintaa

---

# Kannattaako Rustia käyttää?

- jos haluaa syventää ohjelmontiosaamistaan, Rust on hyvä valinta
- Rust + WebAssembly mielenkiintoinen maailma
  - WASI tuo WebAssemblyn pilvimaailmaan
- useasti pullonkaula on I/O eikä CPU, jolloin dynaamiset kielet tarpeeksi tehokkaita
- nykyaikana ohjelmointikielen merkitys monesti pienempi, koska tehdään pienempiä servereitä (esim. lambda vs monoliitti)

---

# Resursseja

- https://doc.rust-lang.org/stable/book/
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Blogipostaus virheenkäsittelystä](https://blog.burntsushi.net/rust-error-handling)
- [Rust and WebAssembly](https://rustwasm.github.io/book/)

---

# Rustilla toteutettua

- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [deno](https://github.com/denoland/deno)
- [Servo](https://github.com/servo/servo)
- [AWS-infraa](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)
- kirjoitettu uudestaan paljon C/C++:lla tehtyjä ohjelmia

---

# Uuden kielen oppiminen

- kirjat, blogit, videot, tutoriaalit
- harjoitustehtävät
- **Toteuta projekti!**
- jaksettava selvittää uuden kielen syntaksia ja tapoja tehdä asioita vaikka houkuttaisi palata vanhaan ja tuttuun
