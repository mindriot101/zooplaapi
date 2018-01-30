extern crate dotenv;
extern crate zooplaapi;

use std::env;
use zooplaapi::{Result, Zoopla, ZooplaQuerySettings};

fn main() {
    dotenv::dotenv().ok();
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let zoopla_key = env::var("ZOOPLA_KEY")?;
    let mut api = Zoopla::new_session(&zoopla_key)?;
    let properties = api.properties(ZooplaQuerySettings::default())?;
    for property in properties.listing {
        println!("Property: {:?}", property);
    }
    Ok(())
}
