use std::{cmp::Ordering, fmt::Display, num::ParseIntError, str::FromStr};

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
/// Errors produced from Date struct
pub enum DateError {
    #[error("Provided value does not form a valid date")]
    /// Provided value is out of bounds and does not form a valid date
    ValueOutOfBounds,
    #[error("Invalid date format, must be: %d-%m-%Y")]
    /// Format of the date string is incorrect
    InvalidFormat,
    #[error("Could not parse to integer: {0}")]
    /// String is not a valid integer
    InvalidValue(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
/// Struct representing a date
pub struct Date {
    day: u8,
    month: u8,
    year: u64,
}

impl Date {
    /// Create new Date object from a day, month, and year
    pub fn new(day: u8, month: u8, year: u64) -> Self {
        Date { day, month, year }
    }

    /// Get the day from this date
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Get the month from this date
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Get the year from this date
    pub fn year(&self) -> u64 {
        self.year
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}-{:0>2}-{}", self.day, self.month, self.year)
    }
}

impl TryFrom<&Date> for NaiveDateTime {
    type Error = DateError;

    fn try_from(date: &Date) -> Result<Self, Self::Error> {
        NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32)
            .ok_or(DateError::ValueOutOfBounds)?
            .and_hms_opt(0, 0, 0)
            .ok_or(DateError::ValueOutOfBounds)
    }
}

impl From<NaiveDateTime> for Date {
    fn from(date: NaiveDateTime) -> Self {
        #[allow(clippy::unwrap_used)]
        let day: u8 = date.day().try_into().unwrap();
        #[allow(clippy::unwrap_used)]
        let month: u8 = date.month().try_into().unwrap();
        let year = date.year();

        Date::new(day, month, year as u64)
    }
}

impl From<Date> for String {
    fn from(date: Date) -> Self {
        date.to_string()
    }
}

lazy_static! {
    static ref DATE_PATTERN: Regex = {
        #[allow(clippy::unwrap_used)]
        Regex::new(r"^(\d{2})-(\d{2})-(\d+)$").unwrap()
    };
}

impl FromStr for Date {
    type Err = DateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = DATE_PATTERN.captures(s).ok_or(DateError::InvalidFormat)?;

        let day = captures
            .get(1)
            .ok_or(DateError::InvalidFormat)?
            .as_str()
            .parse::<u8>()?;
        let month = captures
            .get(2)
            .ok_or(DateError::InvalidFormat)?
            .as_str()
            .parse::<u8>()?;
        let year = captures
            .get(3)
            .ok_or(DateError::InvalidFormat)?
            .as_str()
            .parse::<u64>()?;

        Ok(Date::new(day, month, year))
    }
}

impl TryFrom<String> for Date {
    type Error = DateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => self.day.cmp(&other.day),
                o => o,
            },
            o => o,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use proptest::prelude::*;

    proptest!(
        #[test]
        fn from_str(d in 1u8..32, m in 1u8..13, y: u64) {
            let date_string = format!("{:0>2}-{:0>2}-{}", d, m, y);
            println!("{}", date_string);
            let date = date_string.parse::<Date>().unwrap();

            assert_eq!(date.day(), d);
            assert_eq!(date.month(), m);
            assert_eq!(date.year(), y);
        }

        #[test]
        fn to_string(d in 1u8..32, m in 1u8..13, y: u64) {
            let date = Date::new(d, m, y);

            assert_eq!(date.to_string(), format!("{:0>2}-{:0>2}-{}", d, m, y));
        }
    );

    #[test]
    fn ord() {
        let a = Date::new(1, 3, 2022);
        let b = Date::new(2, 3, 2022);
        let c = Date::new(2, 3, 2022);
        let d = Date::new(1, 4, 2022);

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(b.cmp(&c), Ordering::Equal);

        assert_eq!(b.cmp(&d), Ordering::Less);
    }
}
