use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TimeFrame {
    OneMinute,
    TwoMinutes,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    OneDay,
    TwoDays,
    FiveDays,
    OneWeek,
    TwoWeeks,
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
}

impl TimeFrame {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeFrame::OneMinute => "1m",
            TimeFrame::TwoMinutes => "2m",
            TimeFrame::FiveMinutes => "5m",
            TimeFrame::FifteenMinutes => "15m",
            TimeFrame::OneHour => "1h",
            TimeFrame::OneDay => "1d",
            TimeFrame::TwoDays => "2d",
            TimeFrame::FiveDays => "5d",
            TimeFrame::OneWeek => "1w",
            TimeFrame::TwoWeeks => "2w",
            TimeFrame::OneMonth => "1M",
            TimeFrame::ThreeMonths => "3M",
            TimeFrame::SixMonths => "6M",
            TimeFrame::OneYear => "1Y",
        }
    }
}

impl fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
