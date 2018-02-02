pub mod connection;
pub mod models;
pub mod schema;

use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use self::models::{House, NewHouse};

pub fn create_house(conn: &PgConnection, price: i32) -> House {
    use self::schema::houses;

    let new_house = NewHouse { price: price };

    insert_into(houses::table)
        .values(&new_house)
        .get_result(conn)
        .expect("error saving new house")
}
