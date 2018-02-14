use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use result::Result;
use dotenv::dotenv;

fn fetch_database_url(test: bool) -> String {
    let existing = env::var("DATABASE_URL").unwrap();
    if test {
        let database_url = format!("{}_test", existing);
        database_url
    } else {
        existing
    }
}

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = fetch_database_url(false);
    Ok(PgConnection::establish(&database_url).expect("connecting to database"))
}

pub(crate) fn establish_test_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = fetch_database_url(true);
    Ok(PgConnection::establish(&database_url).expect("connecting to database"))
}

pub(crate) fn initialize(connection: &PgConnection) {
    use std::process::{Command, Stdio};
    use std::fs::File;

    let database_url = fetch_database_url(true);
    let database_name = database_url.split('/').last().unwrap();
    let file = File::open("dumps/schema.sql").unwrap();

    Command::new("psql")
        .arg(database_name)
        .stdin(file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
}
