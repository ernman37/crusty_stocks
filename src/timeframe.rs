use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum TimeFrame {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
    OneWeek,
    OneMonth,
}

impl TimeFrame {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeFrame::OneMinute => "1m",
            TimeFrame::ThreeMinutes => "3m",
            TimeFrame::FiveMinutes => "5m",
            TimeFrame::FifteenMinutes => "15m",
            TimeFrame::ThirtyMinutes => "30m",
            TimeFrame::OneHour => "1h",
            TimeFrame::FourHours => "4h",
            TimeFrame::OneDay => "1d",
            TimeFrame::OneWeek => "1w",
            TimeFrame::OneMonth => "1M",
        }
    }
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for TimeFrame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1m" => Ok(TimeFrame::OneMinute),
            "3m" => Ok(TimeFrame::ThreeMinutes),
            "5m" => Ok(TimeFrame::FiveMinutes),
            "15m" => Ok(TimeFrame::FifteenMinutes),
            "30m" => Ok(TimeFrame::ThirtyMinutes),
            "1h" => Ok(TimeFrame::OneHour),
            "4h" => Ok(TimeFrame::FourHours),
            "1d" => Ok(TimeFrame::OneDay),
            "1w" => Ok(TimeFrame::OneWeek),
            "1M" => Ok(TimeFrame::OneMonth),
            _ => Err(format!(
                "unknown timeframe `{s}`, expected one of: 1m, 3m, 5m, 15m, 30m, 1h, 4h, 1d, 1w, 1M"
            )),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TimeFrame {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TimeFrame {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
