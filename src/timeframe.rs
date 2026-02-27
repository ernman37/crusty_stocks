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
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
