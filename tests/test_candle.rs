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

#[cfg(feature = "csv")]
#[test]
fn candle_as_csv() {
    let c = Candle::new(
        "XRP".to_string(),
        0.5,
        0.6,
        0.7,
        0.4,
        10000,
        1625097600,
        TimeFrame::OneMinute,
    )
    .unwrap();

    let csv = c.to_csv();
    assert!(csv.contains("XRP"));
    assert!(csv.contains("0.5"));
    assert!(csv.contains("0.6"));
    assert!(csv.contains("0.7"));
    assert!(csv.contains("0.4"));
    assert!(csv.contains("10000"));
    assert!(csv.contains("1625097600"));
    assert!(csv.contains("1m"));
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv() {
    let csv = "XRP,0.2,0.9,0.1,0.5,10000,1625097600,1m";
    let c = Candle::from_csv(csv).unwrap();
    assert_eq!(c.ticker, "XRP");
    assert_eq!(c.open, 0.2);
    assert_eq!(c.high, 0.9);
    assert_eq!(c.low, 0.1);
    assert_eq!(c.close, 0.5);
    assert_eq!(c.volume, 10000);
    assert_eq!(c.timestamp, 1625097600);
    assert_eq!(c.timeframe, TimeFrame::OneMinute);
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_length() {
    let csv = "XRP,0.2,0.9,0.1,0.5,10000,1625097600";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
    let csv = "XRP,0.2,0.9,0.1,0.5,10000,1625097600,1m,extra";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_open() {
    let csv = "XRP,invalid,0.9,0.1,0.5,10000,1625097600,1m";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_high() {
    let csv = "XRP,0.2,invalid,0.1,0.5,10000,1625097600,1m";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_low() {
    let csv = "XRP,0.2,0.9,invalid,0.5,10000,1625097600,1m";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_close() {
    let csv = "XRP,0.2,0.9,0.1,invalid,10000,1625097600,1m";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_volume() {
    let csv = "XRP,0.2,0.9,0.1,0.5,invalid,1625097600,1m";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_timestamp() {
    let csv = "XRP,0.2,0.9,0.1,0.5,10000,invalid,1m";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}

#[cfg(feature = "csv")]
#[test]
fn candle_from_csv_invalid_timeframe() {
    let csv = "XRP,0.2,0.9,0.1,0.5,10000,1625097600,invalid";
    let c = Candle::from_csv(csv);
    assert!(c.is_err());
}
