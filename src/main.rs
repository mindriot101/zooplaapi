extern crate diesel;
extern crate dotenv;
extern crate zooplaapi;

use zooplaapi::run;

fn main() {
    dotenv::dotenv().ok();
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
