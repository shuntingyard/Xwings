use from_file::FromFile;
use from_file_derive::FromFile;
use serde_derive::Deserialize;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Deserialize, FromFile, Debug, PartialEq)]
pub struct CredsJson {
    oauth_token: String,
    oauth_token_secret: String,
    user_id: String,
    screen_name: String,
    consumer_key: String,
    consumer_secret: String,
}

#[derive(Deserialize, FromFile, Debug, PartialEq)]
pub struct CredsYaml {
    app: String,
    api_key: String,
    api_key_secret: String,
    bearer_token: String,
    access_token: String,
    access_token_secret: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
