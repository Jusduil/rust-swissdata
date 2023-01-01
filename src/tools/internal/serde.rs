pub mod option_date_dd_mm_yyyyy_dotted {
    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::Date;

    const FORMAT: &'static str = "%d.%m.%Y";

    pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_some(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        s.map(|s| Date::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom))
            .transpose()
    }
}
pub mod date_dd_mm_yyyyy_dotted {
    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::Date;

    const FORMAT: &'static str = "%d.%m.%Y";

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
