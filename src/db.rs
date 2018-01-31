use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use result::Result;
use dotenv::dotenv;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&database_url).expect("connecting to database"))
}
