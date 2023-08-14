use std::path::PathBuf;

use from_file::{FromFile, FromFileError};
use from_file_derive::FromFile;
use serde_derive::Deserialize;
use thiserror::Error;

/// A structure holding consumer_key, access_token and their respective secrets.
#[derive(Debug)]
pub struct OAuth1Keys {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    token_secret: String,
} // TODO: use cases where it's mandatory to store secrets encrypted?

#[derive(Error, Debug)]
pub enum FileToOAuth1KeysError {
    #[error("path buf to str failed")]
    PathNotStrError,
    #[error("from file error")]
    FileError(FromFileError),
}

impl TryFrom<PathBuf> for OAuth1Keys {
    type Error = FileToOAuth1KeysError;

    /// Try to read consumer_key, consumer_secret, access_token, and token_secret from a file.
    fn try_from(infile: PathBuf) -> Result<Self, Self::Error> {
        let infile = infile
            .to_str()
            .ok_or(FileToOAuth1KeysError::PathNotStrError)?;

        // Start with our not-so-legacy format.
        match CredsJson::from_file(infile) {
            Ok(j) => Ok(j.into()),
            Err(e) => match e {
                // We failed, so assume it's our real legacy format.
                FromFileError::SerdeError(_) => match CredsYaml::from_file(infile) {
                    Ok(y) => Ok(y.into()),
                    Err(e) => Err(FileToOAuth1KeysError::FileError(e)),
                },
                _ => Err(FileToOAuth1KeysError::FileError(e)),
            },
        }
    }
}

impl From<CredsJson> for OAuth1Keys {
    fn from(item: CredsJson) -> OAuth1Keys {
        OAuth1Keys {
            access_token: item.oauth_token,
            token_secret: item.oauth_token_secret,
            consumer_key: item.consumer_key,
            consumer_secret: item.consumer_secret,
        }
    }
}

impl From<CredsYaml> for OAuth1Keys {
    fn from(item: CredsYaml) -> OAuth1Keys {
        OAuth1Keys {
            access_token: item.access_token,
            token_secret: item.access_token_secret,
            consumer_key: item.api_key,
            consumer_secret: item.api_key_secret,
        }
    }
}

#[derive(Deserialize, FromFile, PartialEq)]
struct CredsJson {
    oauth_token: String,
    oauth_token_secret: String,
    user_id: String,
    screen_name: String,
    consumer_key: String,
    consumer_secret: String,
}

#[derive(Deserialize, FromFile, PartialEq)]
struct CredsYaml {
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
    fn invalid_extension() {
        let result = OAuth1Keys::try_from(PathBuf::from("keyfile.badextension"));
        // FIXME: don't know how to assert_eq!() without PartialEq...
        println!("{result:?}");
        assert!(false);
    }

    // TODO: add more lib unit tests once the PartialEq problem is solved.
}
