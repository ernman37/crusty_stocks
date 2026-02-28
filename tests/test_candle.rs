use crusty_stocks::{Candle, Error, TimeFrame};

#[test]
fn candle_new_and_helpers() {
    let c = Candle::new(
        "AAPL".to_string(),
        150.0,
        155.0,
        156.0,
        149.0,
        1_000_000,
        1625097600,
        TimeFrame::OneDay,
    )
    .expect("valid candle");

    assert!(c.is_bullish());
    assert!(!c.is_bearish());
    assert_eq!(c.range(), 7.0);
    assert_eq!(c.body(), 5.0);
}

#[test]
fn candle_new_invalid_high_low() {
    let res = Candle::new(
        "AAPL".to_string(),
        10.0,
        9.0,
        5.0,
        6.0,
        100,
        1625097600,
        TimeFrame::OneDay,
    );
    assert!(matches!(res, Err(Error::InvalidCandle(_))));
}

#[test]
fn candle_display() {
    let c = Candle::new(
        "BTC".to_string(),
        20000.0,
        20100.0,
        20200.0,
        19950.0,
        12345,
        1625097600,
        TimeFrame::OneMinute,
    )
    .unwrap();

    let s = format!("{}", c);
    assert!(s.contains("BTC"));
    assert!(s.contains("1m") || s.contains("1m"));
}

#[cfg(feature = "serde")]
#[test]
fn candle_json_serialization() {
    let c = Candle::new(
        "LTC".to_string(),
        150.0,
        155.0,
        156.0,
        149.0,
        10000,
        1625097600,
        TimeFrame::OneMinute,
    )
    .unwrap();

    let json = serde_json::to_string(&c).unwrap();
    assert!(json.contains("LTC"));
    assert!(json.contains("1m"));
}

#[cfg(feature = "serde")]
#[test]
fn candle_json_deserialization() {
    let json = r#"{
        "ticker": "LTC",
        "open": 150.0,
        "close": 155.0,
        "high": 156.0,
        "low": 149.0,
        "volume": 10000,
        "timestamp": 1625097600,
        "timeframe": "1m"
    }"#;

    let c: Candle = serde_json::from_str(json).unwrap();
    assert_eq!(c.ticker, "LTC");
    assert_eq!(c.timeframe, TimeFrame::OneMinute);
}

#[cfg(feature = "yaml")]
#[test]
fn candle_yaml_serialization() {
    let c = Candle::new(
        "ETH".to_string(),
        3000.0,
        3100.0,
        3200.0,
        2900.0,
        5000,
        1625097600,
        TimeFrame::OneHour,
    )
    .unwrap();

    let yaml = serde_yaml::to_string(&c).unwrap();
    assert!(yaml.contains("ETH"));
    assert!(yaml.contains("1h"));
}

#[cfg(feature = "yaml")]
#[test]
fn candle_yaml_deserialization() {
    let yaml = r#"
ticker: ETH
open: 3000.0
close: 3100.0
high: 3200.0
low: 2900.0
volume: 5000
timestamp: 1625097600
timeframe: 1h
"#;

    let c: Candle = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(c.ticker, "ETH");
    assert_eq!(c.timeframe, TimeFrame::OneHour);
}

#[cfg(feature = "yaml")]
#[test]
fn candle_yaml_roundtrip() {
    let original = Candle::new(
        "BTC".to_string(),
        50000.0,
        51000.0,
        52000.0,
        49000.0,
        1000,
        1625097600,
        TimeFrame::OneDay,
    )
    .unwrap();

    let yaml = serde_yaml::to_string(&original).unwrap();
    let roundtripped: Candle = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(original, roundtripped);
}
