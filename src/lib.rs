extern crate dotenv;
extern crate failure;
#[cfg(test)]
extern crate mockito;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::default::Default;
use failure::Error;

pub mod responses;

use responses::HousesResponse;

#[cfg(test)]
pub const URL: &'static str = mockito::SERVER_URL;

#[cfg(not(test))]
pub const URL: &'static str = "https://api.zoopla.co.uk";

pub type Result<T> = std::result::Result<T, Error>;

pub struct ZooplaQuerySettings {
    pub radius: u32,
    pub postcode: String,
    pub listing_status: String,
    pub include_sold: bool,
    pub include_rented: bool,
    pub minimum_price: u32,
    pub maximum_price: u32,
    pub minimum_beds: u32,
    pub property_type: String,
    pub new_homes: bool,
    pub page_size: u32,
}

impl Default for ZooplaQuerySettings {
    fn default() -> Self {
        ZooplaQuerySettings {
            radius: 10,
            postcode: "CV22DX".to_string(),
            listing_status: "sale".to_string(),
            include_sold: false,
            include_rented: false,
            minimum_price: 250000,
            maximum_price: 500000,
            minimum_beds: 4,
            property_type: "houses".to_string(),
            new_homes: false,
            page_size: 10,
        }
    }
}

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

    pub fn properties(&mut self, settings: ZooplaQuerySettings) -> Result<HousesResponse> {
        let include_sold = Self::bool_to_string(settings.include_sold);
        let include_rented = Self::bool_to_string(settings.include_rented);

        let mut resp = self.client
            .get(&self.listings_url())
            .query(&[
                ("api_key", &self.api_key),
                ("radius", &settings.radius.to_string()),
                ("postcode", &settings.postcode),
                ("listing_status", &settings.listing_status),
                ("include_sold", &include_sold),
                ("include_rented", &include_rented),
                ("minimum_price", &settings.minimum_price.to_string()),
                ("maximum_price", &settings.maximum_price.to_string()),
                ("minimum_beds", &settings.minimum_beds.to_string()),
                ("property_type", &settings.property_type),
                ("new_homes", &settings.new_homes.to_string()),
                ("page_size", &settings.page_size.to_string()),
            ])
            .send()?
            .error_for_status()?;

        let result: HousesResponse = resp.json()?;
        Ok(result)
    }

    fn listings_url(&self) -> String {
        format!("{}/api/v1/property_listings.js", URL)
    }

    fn bool_to_string(value: bool) -> String {
        if value {
            "1".to_string()
        } else {
            "0".to_string()
        }
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
        let m = mock(
            "GET",
            Matcher::Regex(r"/api/v1/property_listings.js\?.*".to_string()),
        ).with_body(content)
            .create();
        f();
        m.assert();
    }

    fn send_request<F>(f: F)
    where
        F: Fn(&responses::HousesResponse),
    {
        mock_http(|| {
            dotenv::dotenv().ok();
            let zoopla_key = env::var("ZOOPLA_KEY").unwrap();
            let mut api = Zoopla::new_session(&zoopla_key).unwrap();
            let properties = api.properties(ZooplaQuerySettings::default()).unwrap();
            f(&properties)
        });
    }

    fn first_property<F>(f: F)
    where
        F: Fn(&responses::HouseResponse),
    {
        send_request(|properties| {
            f(&properties.listing[0]);
        });
    }

    #[test]
    fn test_first_property() {
        first_property(|p| {
            assert_eq!(p.price, 500000);
            assert_eq!(p.num_bathrooms, 3);
            assert_eq!(p.num_bedrooms, 6);
            assert_eq!(p.num_floors, 0);
            assert_eq!(p.category, "Residential".to_string());
            assert_eq!(p.property_type, "Detached house".to_string());
        });
    }

}
