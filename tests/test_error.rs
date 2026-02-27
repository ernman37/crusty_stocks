use crusty_stocks::Error;

#[test]
fn error_display() {
    let err = Error::InvalidCandle("test".into());
    assert_eq!(format!("{}", err), "Invalid candle: test");
}
