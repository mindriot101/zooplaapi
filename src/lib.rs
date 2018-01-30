extern crate dotenv;
extern crate failure;
#[cfg(test)]
extern crate mockito;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use failure::Error;

pub mod responses;

use responses::HousesResponse;

#[cfg(test)]
pub const URL: &'static str = mockito::SERVER_URL;

#[cfg(not(test))]
pub const URL: &'static str = "https://api.zoopla.co.uk";

pub type Result<T> = std::result::Result<T, Error>;

pub struct Zoopla {
    pub api_key: String,
    pub client: reqwest::Client,
}

impl Zoopla {
    pub fn new_session(api_key: &str) -> Result<Zoopla> {
        let client = reqwest::Client::new();
        Ok(Zoopla {
            api_key: api_key.to_string(),
            client: client,
        })
    }

    pub fn properties(&mut self) -> Result<HousesResponse> {
        let mut resp = self.client
            .get(&self.listings_url())
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

    fn listings_url(&self) -> String {
        format!("{}/api/v1/property_listings.js", URL)
    }
}

#[cfg(test)]
mod tests {
    use mockito::{mock, Matcher};
    use std::env;
    use super::*;

    fn mock_http<F>(f: F)
    where
        F: Fn(),
    {
        let content = include_str!("../fixtures/houses.json");
        let m = mock("GET", Matcher::Any).with_body(content).create();
        f();
        m.assert();
    }

    #[test]
    fn test_something() {
        mock_http(|| {
            dotenv::dotenv().ok();
            let zoopla_key = env::var("ZOOPLA_KEY").unwrap();
            let mut api = Zoopla::new_session(&zoopla_key).unwrap();
            let properties = api.properties().unwrap();
            let listings = properties.listing;
            assert_eq!(listings.len(), 10);
        });
    }
}
