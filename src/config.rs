use std::str::FromStr;

use http::uri::Uri;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub urls: Vec<RedirectRule>,
}

#[derive(Deserialize, Debug)]
pub struct RedirectRule {
    pub short: String,
    #[serde(deserialize_with = "deserialize_uri")]
    pub long: Uri,
}

fn deserialize_uri<'de, D>(d: D) -> Result<Uri, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)
        .and_then(|s| Uri::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string())))
}
