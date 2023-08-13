use from_file::{FromFile, FromFileError};

use auth::{CredsJson, CredsYaml};

fn main() {
    let keyfile = "keys/twitter_keys.yaml";

    match CredsJson::from_file(keyfile) {
        Ok(p) => println!("Data read: {p:#?}"),
        Err(e) => match e {
            FromFileError::SerdeError(_) => match CredsYaml::from_file(keyfile) {
                Ok(p) => println!("Data read: {p:#?}"),
                _ => eprintln!("{}", e),
            },
            _ => eprintln!("{}", e),
        },
    }
}
