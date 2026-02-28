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

#[cfg(feature = "json")]
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

#[cfg(feature = "json")]
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
