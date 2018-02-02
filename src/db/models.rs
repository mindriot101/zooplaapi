// use super::schema::*;

#[derive(Queryable, Debug)]
pub struct House {
    pub id: i32,
    pub price: i32,
    pub first_published_date: String,
    pub last_published_date: String,
}

#[derive(Queryable, Debug)]
pub struct Category {
    pub id: i32,
    pub property_type: String,
    pub category: String,
    pub price_modifier: Option<String>,
}
