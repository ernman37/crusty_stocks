use std::fmt;

use crate::{Error, TimeFrame};

#[derive(Debug, Clone, PartialEq)]
pub struct Candle {
    pub ticker: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub timeframe: TimeFrame,
}

impl Candle {
    pub fn new(
        ticker: String,
        open: f64,
        close: f64,
        high: f64,
        low: f64,
        volume: f64,
        timeframe: TimeFrame,
    ) -> Result<Self, Error> {
        if high < low {
            return Err(Error::InvalidCandle("high must be >= low".to_string()));
        }
        if volume < 0.0 {
            return Err(Error::InvalidCandle("volume must be non-negative".to_string()));
        }
        Ok(Self {
            ticker,
            open,
            close,
            high,
            low,
            volume,
            timeframe,
        })
    }

    /// Returns `true` if the candle closed higher than it opened.
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Returns `true` if the candle closed lower than it opened.
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    /// The full range of the candle (high - low).
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// The body size of the candle (absolute difference between open and close).
    pub fn body(&self) -> f64 {
        (self.close - self.open).abs()
    }
}

impl fmt::Display for Candle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.0}",
            self.ticker, self.timeframe, self.open, self.high, self.low, self.close, self.volume
        )
    }
}
