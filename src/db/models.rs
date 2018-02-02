use super::schema::*;

#[derive(Insertable)]
#[table_name = "houses"]
pub struct NewHouse<'a> {
    pub price: i32,
    pub first_published_date: &'a str,
    pub last_published_date: &'a str,
}

#[derive(Insertable)]
#[table_name = "houses_agents"]
pub struct NewAgent<'a> {
    pub name: &'a str,
    pub phone: &'a str,
}

#[derive(Insertable)]
#[table_name = "houses_categories"]
pub struct NewCategory<'a> {
    pub property_type: &'a str,
    pub category: &'a str,
    pub price_modifier: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name = "houses_locations"]
pub struct NewLocation<'a> {
    pub longitude: f32,
    pub latitude: f32,
    pub street_name: &'a str,
    pub displayable_address: &'a str,
}

#[derive(Insertable)]
#[table_name = "houses_properties"]
pub struct NewProperty<'a> {
    pub description: &'a str,
    pub num_bathrooms: i32,
    pub num_bedrooms: i32,
    pub num_floors: i32,
}

#[derive(Insertable)]
#[table_name = "houses_urls"]
pub struct NewUrls<'a> {
    pub details: &'a str,
    pub property_report: &'a str,
}
