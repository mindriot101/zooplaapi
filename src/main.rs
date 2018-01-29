extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use std::env;
use failure::Error;

mod responses;

type Result<T> = std::result::Result<T, Error>;

struct Zoopla {
    pub session_id: String,
    pub api_key: String,
    pub client: reqwest::Client,
}

impl Zoopla {
    fn new_session(api_key: &str) -> Result<Zoopla> {
        let mut client = reqwest::Client::new();
        let session_key = Self::get_session_id(&mut client, api_key)?;
        Ok(Zoopla {
            session_id: session_key,
            api_key: api_key.to_string(),
            client: client,
        })
    }

    fn get_session_id(client: &mut reqwest::Client, api_key: &str) -> Result<String> {
        let mut resp = client
            .get("https://api.zoopla.co.uk/api/v1/get_session_id.js")
            .query(&[("api_key", api_key)])
            .send()?;
        if !resp.status().is_success() {
            if let Some(reason) = resp.status().canonical_reason() {
                return Err(format_err!(
                    "error: {}; status: {:?}",
                    reason,
                    resp.status()
                ));
            } else {
                return Err(format_err!("unknown error; status: {:?}", resp.status()));
            }
        }

        let result: responses::SessionResponse = resp.json()?;
        Ok(result.session_id)
    }
}

fn main() {
    dotenv::dotenv().ok();
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let zoopla_key = env::var("ZOOPLA_KEY")?;
    let _api = Zoopla::new_session(&zoopla_key)?;
    Ok(())
}
