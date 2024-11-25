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

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use chrono::NaiveDate;

    #[derive(Serialize, Deserialize)]
    struct Data {
        #[serde(with = "super")]
        date: NaiveDate
    }

    #[test]
    fn test_serialize_deserialize_date() {
        let date_struct = Data {
            date: NaiveDate::default()
        };

        let serialized_date = bincode::serialize(&date_struct).unwrap();

        let deserialized_date: Data = bincode::deserialize(&serialized_date).unwrap();

        assert_eq!(date_struct.date, deserialized_date.date);
    }

    #[test]
    fn test_serialize_deserialize_error() {
        let invalid_bytes = vec![0x00, 0xFF, 0x00];

        let result: Result<Data, bincode::Error> = bincode::deserialize(&invalid_bytes);

        assert!(result.is_err());
    }
}