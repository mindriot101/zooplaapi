#[derive(Queryable, Debug)]
pub struct House {
    pub id: i32,
    pub price: i32,
    pub first_published_date: String,
    pub last_published_date: String,
}
