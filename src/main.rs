extern crate dotenv;
extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use failure::Error;

mod responses;

use responses::{HousesResponse, SessionResponse};

type Result<T> = std::result::Result<T, Error>;

struct Zoopla {
    pub session_id: String,
    pub api_key: String,
    pub client: reqwest::Client,
}

impl Zoopla {
    pub fn new_session(api_key: &str) -> Result<Zoopla> {
        let mut client = reqwest::Client::new();
        let session_key = Self::get_session_id(&mut client, api_key)?;
        Ok(Zoopla {
            session_id: session_key,
            api_key: api_key.to_string(),
            client: client,
        })
    }

    pub fn properties(&mut self) -> Result<HousesResponse> {
        let mut resp = self.client
            .get("https://api.zoopla.co.uk/api/v1/property_listings.js")
            .query(&[
                ("api_key", &self.api_key),
                ("radius", &"10".to_string()),
                ("postcode", &"CV22DX".to_string()),
                ("listing_status", &"sale".to_string()),
                ("include_sold", &"0".to_string()),
                ("include_rented", &"0".to_string()),
                ("minimum_price", &"250000".to_string()),
                ("maximum_price", &"500000".to_string()),
                ("minimum_beds", &"4".to_string()),
                ("property_type", &"houses".to_string()),
                ("new_homes", &"false".to_string()),
                // TODO: increase page size
                ("page_size", &"10".to_string()),
            ])
            .send()?
            .error_for_status()?;

        let result: HousesResponse = resp.json()?;
        Ok(result)
    }

    fn get_session_id(client: &mut reqwest::Client, api_key: &str) -> Result<String> {
        let mut resp = client
            .get("https://api.zoopla.co.uk/api/v1/get_session_id.js")
            .query(&[("api_key", api_key)])
            .send()?
            .error_for_status()?;

        let result: SessionResponse = resp.json()?;
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
    let mut api = Zoopla::new_session(&zoopla_key)?;
    let properties = api.properties()?;
    for property in properties.listing {
        println!("Property: {:?}", property);
    }
    Ok(())
}
