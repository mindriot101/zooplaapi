extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;
#[cfg(test)]
extern crate mockito;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod result;
pub mod zoopla;
mod db;

pub use result::Result;
pub use zoopla::api::{Zoopla, ZooplaQuerySettings};

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
            assert_eq!(p.num_bathrooms, 4);
            assert_eq!(p.num_bedrooms, 4);
            assert_eq!(p.num_floors, 0);
            assert_eq!(p.category, "Residential".to_string());
            assert_eq!(p.property_type, "Detached house".to_string());
        });
    }

}
