use std::str::FromStr;
use serde::de::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct HousesResponse {
    pub listing: Vec<HouseResponse>,
    pub county: String,
    pub bounding_box: BoundingBoxResponse,
}

#[derive(Deserialize, Debug)]
pub struct BoundingBoxResponse {
    #[serde(deserialize_with = "parse_f32")] pub longitude_min: f32,
    #[serde(deserialize_with = "parse_f32")] pub longitude_max: f32,
    #[serde(deserialize_with = "parse_f32")] pub latitude_min: f32,
    #[serde(deserialize_with = "parse_f32")] pub latitude_max: f32,
}

#[derive(Deserialize, Debug)]
pub struct HouseResponse {
    #[serde(deserialize_with = "parse_i64")] pub listing_id: i64,
    #[serde(deserialize_with = "parse_i64")] pub price: i64,
    pub first_published_date: String,
    pub last_published_date: String,
    pub price_change: Option<Vec<PriceChangeResponse>>,
    pub price_change_summary: Option<PriceChangeSummaryResponse>,

    /* Location */
    pub latitude: f32,
    pub longitude: f32,
    pub street_name: String,
    pub displayable_address: String,

    /* Categories */
    pub property_type: String,
    pub category: String,
    pub price_modifier: Option<String>,

    /* Agent */
    pub agent_name: String,
    pub agent_phone: String,

    /* URLs */
    pub details_url: String,
    pub property_report_url: String,
    pub floor_plan: Option<Vec<String>>,

    /* Property */
    pub description: String,
    #[serde(deserialize_with = "parse_i64")] pub num_bathrooms: i64,
    #[serde(deserialize_with = "parse_i64")] pub num_bedrooms: i64,
    #[serde(deserialize_with = "parse_i64")] pub num_floors: i64,
}

#[derive(Deserialize, Debug)]
pub struct PriceChangeResponse {
    pub direction: String,
    pub date: String,
    pub percent: String,
    #[serde(deserialize_with = "parse_i64")] pub price: i64,
}

#[derive(Deserialize, Debug)]
pub struct PriceChangeSummaryResponse {
    pub direction: String,
    pub percent: String,
    pub last_updated_date: String,
}

fn parse_i64<'de, D>(deserializer: D) -> ::std::result::Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(i64::from_str(&s).expect(&format!("converting string {} into integer", s)))
}

fn parse_f32<'de, D>(deserializer: D) -> ::std::result::Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(f32::from_str(&s).expect(&format!("converting string {} into integer", s)))
}
