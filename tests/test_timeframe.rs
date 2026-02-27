use crusty_stocks::TimeFrame;

#[test]
fn timeframe_one_minute_as_str_and_display() {
    let tf = TimeFrame::OneMinute;
    assert_eq!(tf.as_str(), "1m");
    assert_eq!(format!("{}", tf), "1m");
}

#[test]
fn timeframe_two_minutes_as_str_and_display() {
    let tf = TimeFrame::ThreeMinutes;
    assert_eq!(tf.as_str(), "3m");
    assert_eq!(format!("{}", tf), "3m");
}

#[test]
fn timeframe_five_minutes_as_str_and_display() {
    let tf = TimeFrame::FiveMinutes;
    assert_eq!(tf.as_str(), "5m");
    assert_eq!(format!("{}", tf), "5m");
}

#[test]
fn timeframe_fifteen_minutes_as_str_and_display() {
    let tf = TimeFrame::FifteenMinutes;
    assert_eq!(tf.as_str(), "15m");
    assert_eq!(format!("{}", tf), "15m");
}

#[test]
fn timeframe_thirty_minutes_as_str_and_display() {
    let tf = TimeFrame::ThirtyMinutes;
    assert_eq!(tf.as_str(), "30m");
    assert_eq!(format!("{}", tf), "30m");
}

#[test]
fn timeframe_one_hour_as_str_and_display() {
    let tf = TimeFrame::OneHour;
    assert_eq!(tf.as_str(), "1h");
    assert_eq!(format!("{}", tf), "1h");
}

#[test]
fn timeframe_four_hours_as_str_and_display() {
    let tf = TimeFrame::FourHours;
    assert_eq!(tf.as_str(), "4h");
    assert_eq!(format!("{}", tf), "4h");
}

#[test]
fn timeframe_one_day_as_str_and_display() {
    let tf = TimeFrame::OneDay;
    assert_eq!(tf.as_str(), "1d");
    assert_eq!(format!("{}", tf), "1d");
}

#[test]
fn timeframe_one_week_as_str_and_display() {
    let tf = TimeFrame::OneWeek;
    assert_eq!(tf.as_str(), "1w");
    assert_eq!(format!("{}", tf), "1w");
}

#[test]
fn timeframe_one_month_as_str_and_display() {
    let tf = TimeFrame::OneMonth;
    assert_eq!(tf.as_str(), "1M");
    assert_eq!(format!("{}", tf), "1M");
}
