use chrono::{DateTime, Utc};
use serde::{Deserialize, de::Deserializer, ser::Serializer};
use std::time::SystemTime;
pub fn deserialize_system_time<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let datetime = DateTime::parse_from_rfc3339(s).map_err(serde::de::Error::custom)?;
    Ok(datetime.with_timezone(&Utc).into())
}

pub fn serialize_system_time<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime: DateTime<Utc> = time.clone().into(); // SystemTime'ı DateTime<Utc>'ye çevir
    let datetime_str = datetime.to_rfc3339(); // DateTime<Utc>'yi string'e çevir
    serializer.serialize_str(&datetime_str) // String'i serileştir
}