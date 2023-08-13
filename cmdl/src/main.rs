use from_file::FromFile;

use auth::Credentials;

fn main() {
    match Credentials::from_file("keys/twitter_keys.json") {
        Ok(p) => println!("Data read: {p:#?}"),
        Err(e) => eprintln!("{}", e),
    }
}
