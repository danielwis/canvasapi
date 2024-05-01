use serde::{Deserialize, Deserializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub(crate) fn deserialize_optional_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let val: Option<String> = Option::deserialize(deserializer)?;
    Ok(match val {
        Some(s) => {
            if s == "null" {
                None
            } else {
                Some(OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)?)
            }
        }
        None => None,
    })
}
