pub mod connection;
pub mod models;
pub mod schema;

use diesel::prelude::*;

pub fn foo() {
    use db::schema::houses::dsl::*;
    use db::schema::houses_categories::dsl::*;

    let connection = connection::establish_connection().expect("connecting to database");
    connection
        .transaction::<_, ::diesel::result::Error, _>(|| {
            let results: Vec<(String, i32)> = houses
                .filter(price.eq(500000))
                .inner_join(houses_categories)
                .select((property_type, price))
                .load(&connection)
                .expect("error loading houses");

            for value in results {
                println!("{:?}", value);
            }

            Ok(())
        })
        .expect("failed to run transaction");
}
