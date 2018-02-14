#[macro_use]
extern crate diesel;
extern crate dotenv;
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
pub mod db;

use std::env;
pub use result::Result;
pub use zoopla::api::{Zoopla, ZooplaQuerySettings};
use db::create_property;
use diesel::prelude::*;

pub fn run() -> Result<()> {
    use db::schema::*;
    let connection = db::connection::establish_test_connection()?;
    Ok(())
    // let zoopla_key = env::var("ZOOPLA_KEY")?;
    // let connection = db::connection::establish_connection()?;
    // let mut api = Zoopla::new_session(&zoopla_key)?;
    // let properties = api.properties(ZooplaQuerySettings {
    //     ..Default::default()
    // })?;
    // for property in properties.listing {
    //     connection.transaction::<_, ::diesel::result::Error, _>(|| {
    //         create_property(&property, &connection);
    //         Ok(())
    //     })?;
    // }
    // Ok(())
}

#[cfg(test)]
mod tests {
    use mockito::{mock, Matcher};
    use std::env;
    use super::*;
    use diesel::prelude::*;
    use super::zoopla::responses;
    use super::db::connection::{establish_test_connection, initialize};
    use super::db::create_property;
    use super::db::schema::*;

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

    fn insert<F>(f: F)
    where
        F: Fn(&PgConnection),
    {
        let connection = establish_test_connection().unwrap();
        initialize(&connection);
        first_property(|p| {
            connection.test_transaction::<_, ::diesel::result::Error, _>(|| {
                create_property(p, &connection);
                f(&connection);
                Ok(())
            });
        });
    }

    #[test]
    fn test_insert_house() {
        insert(|conn| assert_eq!(houses::table.count().get_result(conn), Ok(1)));
    }

    #[test]
    fn test_insert_agent() {
        insert(|conn| assert_eq!(houses_agents::table.count().get_result(conn), Ok(1)));
    }

    #[test]
    fn test_insert_categories() {
        insert(|conn| assert_eq!(houses_categories::table.count().get_result(conn), Ok(1)));
    }

    #[test]
    fn test_insert_locations() {
        insert(|conn| assert_eq!(houses_locations::table.count().get_result(conn), Ok(1)));
    }

    #[test]
    fn test_insert_property() {
        insert(|conn| assert_eq!(houses_properties::table.count().get_result(conn), Ok(1)));
    }

    #[test]
    fn test_insert_urls() {
        insert(|conn| assert_eq!(houses_urls::table.count().get_result(conn), Ok(1)));
    }
}
