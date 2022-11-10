use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub fn de_from_str_to_float<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    return f32::from_str(&s).map_err(de::Error::custom);
}

pub fn de_from_str_to_int<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    return i64::from_str(&s).map_err(de::Error::custom);
}