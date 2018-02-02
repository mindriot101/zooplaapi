use super::schema::houses;

#[derive(Queryable)]
pub struct House {
    pub id: i32,
    pub price: i32,
}

#[derive(Insertable)]
#[table_name = "houses"]
pub struct NewHouse {
    pub price: i32,
}
