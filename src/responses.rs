use std::str::FromStr;
use serde::de::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub(crate) struct SessionResponse {
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct HousesResponse {
    pub listing: Vec<HouseResponse>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct HouseResponse {
    #[serde(deserialize_with = "parse_i64")] num_bedrooms: i64,
    latitude: f32,
    longitude: f32,
    property_type: String,
    description: String,
    price: String,
    listing_id: String,
    agent_name: String,
    first_published_date: String,
    displayable_address: String,
    price_modifier: Option<String>,
    floor_plan: Option<Vec<String>>,
    street_name: String,
    num_bathrooms: String,
    // TODO: add this
    // price_change: ...,
    last_published_date: String,
    // TODO: add this
    // price_change_summary: ...,
}

fn parse_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(i64::from_str(&s).unwrap())
}
