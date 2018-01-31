use std::str::FromStr;
use serde::de::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct HousesResponse {
    pub listing: Vec<HouseResponse>,
}

#[derive(Deserialize, Debug)]
pub struct HouseResponse {
    pub latitude: f32,
    pub longitude: f32,
    pub property_type: String,
    pub description: String,
    pub agent_name: String,
    pub first_published_date: String,
    pub displayable_address: String,
    pub price_modifier: Option<String>,
    pub floor_plan: Option<Vec<String>>,
    pub street_name: String,
    #[serde(deserialize_with = "parse_i64")] pub listing_id: i64,
    #[serde(deserialize_with = "parse_i64")] pub num_bathrooms: i64,
    #[serde(deserialize_with = "parse_i64")] pub num_bedrooms: i64,
    #[serde(deserialize_with = "parse_i64")] pub price: i64,
    pub last_published_date: String,
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
