extern crate dotenv;
extern crate zooplaapi;

use zooplaapi::Result;
// use zooplaapi::db::create_house;

fn main() {
    dotenv::dotenv().ok();
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    use zooplaapi::db::foo;

    foo();

    /*
    let zoopla_key = env::var("ZOOPLA_KEY")?;
    let connection = establish_connection()?;
    let mut api = Zoopla::new_session(&zoopla_key)?;
    let properties = api.properties(ZooplaQuerySettings {
        ..Default::default()
    })?;
    for property in properties.listing {
        // create_house(&connection, property.price as _);
    }
    */
    Ok(())
}
