use std::{fmt::Display, num::ParseIntError, str::FromStr};

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
    year: i32,
}

impl Date {
    /// Create new Date object from a day, month, and year
    pub fn new(day: u8, month: u8, year: i32) -> Self {
        Date { day, month, year }
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
        NaiveDate::from_ymd_opt(date.year, date.month as u32, date.day as u32)
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

        Date::new(day, month, year)
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
        Regex::new(r"(\d{2})-(\d{2})-(\d{4})").unwrap()
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
            .parse::<i32>()?;

        Ok(Date::new(day, month, year))
    }
}

impl TryFrom<String> for Date {
    type Error = DateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn to_string() {
        let date = Date::new(6, 7, 2004);

        assert_eq!(date.to_string(), "06-07-2004");
    }

    #[test]
    fn from_str() {
        let date = "01-03-2024".parse::<Date>().unwrap();

        assert!(date == Date::new(1, 3, 2024))
    }
}
