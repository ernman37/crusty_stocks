use std::fmt;

use crate::{Error, TimeFrame};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Candle {
    pub ticker: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u32,
    pub timestamp: u32,
    pub timeframe: TimeFrame,
}

impl Candle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ticker: String,
        open: f64,
        close: f64,
        high: f64,
        low: f64,
        volume: u32,
        timestamp: u32,
        timeframe: TimeFrame,
    ) -> Result<Self, Error> {
        if high < low {
            return Err(Error::InvalidCandle("high must be >= low".to_string()));
        }
        Ok(Self {
            ticker,
            open,
            close,
            high,
            low,
            volume,
            timestamp,
            timeframe,
        })
    }

    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }

    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    pub fn body(&self) -> f64 {
        (self.close - self.open).abs()
    }

    #[cfg(feature = "csv")]
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.ticker,
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume,
            self.timestamp,
            self.timeframe.as_str()
        )
    }

    #[cfg(feature = "csv")]
    pub fn from_csv(csv: &str) -> Result<Self, Error> {
        let mut parts = csv.split(',');
        if parts.clone().count() != 8 {
            return Err(Error::InvalidCandle("Invalid CSV format".to_string()));
        }

        let ticker = parts.next().unwrap().to_string();
        let open = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidCandle("Invalid open price".to_string()))?;
        let high = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidCandle("Invalid high price".to_string()))?;
        let low = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidCandle("Invalid low price".to_string()))?;
        let close = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidCandle("Invalid close price".to_string()))?;
        let volume = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidCandle("Invalid volume".to_string()))?;
        let timestamp = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| Error::InvalidCandle("Invalid timestamp".to_string()))?;
        let timeframe = TimeFrame::from_str(parts.next().unwrap())
            .ok_or_else(|| Error::InvalidCandle("Invalid timeframe".to_string()))?;

        Candle::new(ticker, open, close, high, low, volume, timestamp, timeframe)
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
