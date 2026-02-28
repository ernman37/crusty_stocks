use std::fmt;

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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1m" => Some(TimeFrame::OneMinute),
            "3m" => Some(TimeFrame::ThreeMinutes),
            "5m" => Some(TimeFrame::FiveMinutes),
            "15m" => Some(TimeFrame::FifteenMinutes),
            "30m" => Some(TimeFrame::ThirtyMinutes),
            "1h" => Some(TimeFrame::OneHour),
            "4h" => Some(TimeFrame::FourHours),
            "1d" => Some(TimeFrame::OneDay),
            "1w" => Some(TimeFrame::OneWeek),
            "1M" => Some(TimeFrame::OneMonth),
            _ => None,
        }
    }
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
        TimeFrame::from_str(&s).ok_or_else(|| {
            serde::de::Error::unknown_variant(
                &s,
                &["1m", "3m", "5m", "15m", "30m", "1h", "4h", "1d", "1w", "1M"],
            )
        })
    }
}
