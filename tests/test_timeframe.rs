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

#[test]
fn timeframe_from_str() {
    assert_eq!(TimeFrame::from_str("1m"), Some(TimeFrame::OneMinute));
    assert_eq!(TimeFrame::from_str("3m"), Some(TimeFrame::ThreeMinutes));
    assert_eq!(TimeFrame::from_str("5m"), Some(TimeFrame::FiveMinutes));
    assert_eq!(TimeFrame::from_str("15m"), Some(TimeFrame::FifteenMinutes));
    assert_eq!(TimeFrame::from_str("30m"), Some(TimeFrame::ThirtyMinutes));
    assert_eq!(TimeFrame::from_str("1h"), Some(TimeFrame::OneHour));
    assert_eq!(TimeFrame::from_str("4h"), Some(TimeFrame::FourHours));
    assert_eq!(TimeFrame::from_str("1d"), Some(TimeFrame::OneDay));
    assert_eq!(TimeFrame::from_str("1w"), Some(TimeFrame::OneWeek));
    assert_eq!(TimeFrame::from_str("1M"), Some(TimeFrame::OneMonth));
    assert_eq!(TimeFrame::from_str("invalid"), None);
}

#[cfg(feature = "serde")]
#[test]
fn timeframe_json_serialization() {
    let tf = TimeFrame::OneMinute;
    let json = serde_json::to_string(&tf).unwrap();
    assert!(json.contains("1m"));
}

#[cfg(feature = "serde")]
#[test]
fn timeframe_json_deserialization() {
    let json = r#""1m""#;
    let tf: TimeFrame = serde_json::from_str(json).unwrap();
    assert_eq!(tf, TimeFrame::OneMinute);
}

#[cfg(feature = "serde")]
#[test]
fn timeframe_json_deserialization_invalid_returns_err() {
    let json = r#""invalid""#;
    let result: Result<TimeFrame, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[cfg(feature = "serde")]
#[test]
fn timeframe_json_deserialization_wrong_type_returns_err() {
    let json = r#"123"#;
    let result: Result<TimeFrame, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[cfg(feature = "yaml")]
#[test]
fn timeframe_yaml_serialization() {
    let tf = TimeFrame::OneDay;
    let yaml = serde_yaml::to_string(&tf).unwrap();
    assert!(yaml.contains("1d"));
}

#[cfg(feature = "yaml")]
#[test]
fn timeframe_yaml_deserialization() {
    let yaml = r#"1d"#;
    let tf: TimeFrame = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(tf, TimeFrame::OneDay);
}

#[cfg(feature = "yaml")]
#[test]
fn timeframe_yaml_deserialization_invalid_returns_err() {
    let yaml = r#"invalid"#;
    let result: Result<TimeFrame, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err());
}

#[cfg(feature = "yaml")]
#[test]
fn timeframe_yaml_roundtrip() {
    let variants = [
        TimeFrame::OneMinute,
        TimeFrame::ThreeMinutes,
        TimeFrame::FiveMinutes,
        TimeFrame::FifteenMinutes,
        TimeFrame::ThirtyMinutes,
        TimeFrame::OneHour,
        TimeFrame::FourHours,
        TimeFrame::OneDay,
        TimeFrame::OneWeek,
        TimeFrame::OneMonth,
    ];
    for tf in &variants {
        let yaml = serde_yaml::to_string(tf).unwrap();
        let roundtripped: TimeFrame = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(tf, &roundtripped);
    }
}
