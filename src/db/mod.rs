pub mod connection;
pub mod models;
pub mod schema;

use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use self::models::*;

// use self::models::{House, NewHouse};

/*
pub fn create_house(conn: &PgConnection, price: i32) -> House {
    use self::schema::houses;

    let new_house = NewHouse { price: price };

    insert_into(houses::table)
        .values(&new_house)
        .get_result(conn)
        .expect("error saving new house")
}
*/

pub fn foo() {
    use db::schema::houses::dsl::*;
    use db::schema::houses_categories::dsl::*;

    let connection = connection::establish_connection().expect("connecting to database");
    let results: Vec<(String, i32)> = houses
        .filter(price.eq(500000))
        .inner_join(houses_categories)
        .select((property_type, price))
        .load(&connection)
        .expect("error loading houses");

    for value in results {
        println!("{:?}", value);
    }
}
