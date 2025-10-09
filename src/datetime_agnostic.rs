use chrono::{DateTime, TimeZone, Utc};
use serde::{
  de::{self, Deserializer, Visitor},
  Deserialize,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct FlexibleDateTime(pub DateTime<Utc>);

impl<'de> Deserialize<'de> for FlexibleDateTime {
  fn deserialize<D>(d: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    datetime_from_int_or_str(d).map(FlexibleDateTime)
  }
}

pub fn datetime_from_int_or_str<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  struct V;
  impl<'de> Visitor<'de> for V {
    type Value = DateTime<Utc>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
      f.write_str("unix seconds/millis or RFC3339 datetime string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      // Heuristic: treat big values as milliseconds.
      let (secs, nanos) = if v.abs() >= 1_000_000_000_000 {
        (v / 1000, ((v % 1000) * 1_000_000) as i32)
      } else {
        (v, 0)
      };
      Utc
        .timestamp_opt(secs, nanos as u32)
        .single()
        .ok_or_else(|| E::custom("invalid UNIX timestamp"))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      let (secs, nanos) = if v >= 1_000_000_000_000 {
        (v / 1000, ((v % 1000) * 1_000_000) as u32)
      } else {
        (v, 0)
      };
      Utc
        .timestamp_opt(secs as i64, nanos)
        .single()
        .ok_or_else(|| E::custom("invalid UNIX timestamp"))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(E::custom)
    }
  }

  d.deserialize_any(V)
}
