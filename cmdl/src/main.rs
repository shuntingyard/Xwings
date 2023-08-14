use std::{env, path::PathBuf};

use auth::OAuth1Keys;

fn main() {
    let keyfile = PathBuf::from(env::args().nth(1).expect("Arg1: keyfile expected"));

    match OAuth1Keys::try_from(keyfile) {
        Ok(p) => println!("Data read: {p:#?}"),
        Err(e) => eprintln!("{e:?}"),
    }
}
