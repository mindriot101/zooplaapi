use std::str::FromStr;
use serde::de::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct HousesResponse {
    pub listing: Vec<HouseResponse>,
}

#[derive(Deserialize, Debug)]
pub struct HouseResponse {
    #[serde(deserialize_with = "parse_i64")] num_bedrooms: i64,
    latitude: f32,
    longitude: f32,
    property_type: String,
    description: String,
    agent_name: String,
    first_published_date: String,
    displayable_address: String,
    price_modifier: Option<String>,
    floor_plan: Option<Vec<String>>,
    street_name: String,
    #[serde(deserialize_with = "parse_i64")] listing_id: i64,
    #[serde(deserialize_with = "parse_i64")] num_bathrooms: i64,
    #[serde(deserialize_with = "parse_i64")] price: i64,
    last_published_date: String,
    // TODO: add this
    // price_change: ...,
    // TODO: add this
    // price_change_summary: ...,
}

fn parse_i64<'de, D>(deserializer: D) -> ::std::result::Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(i64::from_str(&s).expect(&format!("converting string {} into integer", s)))
}
