use chrono::NaiveDate;
use serde::{self, Deserialize, Serializer, Deserializer};

const FORMAT: &'static str = "%d/%m/%Y";

pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDate::parse_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)?;

    Ok(dt)
}