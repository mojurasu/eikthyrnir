use std::{fmt, io};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use chrono::NaiveDateTime;
use regex::{Match, Regex};
use walkdir::WalkDir;

use crate::error;

const DURATION_PATTERN: &str = r#"(?P<duration>\d+)(?P<unit>[smhdw])"#;

#[derive(Debug)]
enum Unit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}

impl Unit {
    pub fn as_seconds(&self) -> u64 {
        match self {
            Unit::Second => 1,
            Unit::Minute => 60,
            Unit::Hour => 60 * 60,
            Unit::Day => 60 * 60 * 24,
            Unit::Week => 60 * 60 * 60 * 7,
        }
    }
}

impl From<&str> for Unit {
    fn from(s: &str) -> Self {
        match s {
            "s" => Unit::Second,
            "m" => Unit::Minute,
            "h" => Unit::Hour,
            "d" => Unit::Day,
            "w" => Unit::Week,
            _ => Unit::Second,
        }
    }
}


#[derive(Debug)]
pub struct Duration {
    string: String,
    pub seconds: u64,
}

impl FromStr for Duration {
    type Err = error::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut seconds = 0;
        let re = Regex::new(DURATION_PATTERN)?;
        for g in re.captures_iter(string) {
            let unit: Unit = g.name("unit").unwrap().as_str().into();
            let duration: u64 = g.name("duration").unwrap().as_str().parse().unwrap();
            seconds += duration * unit.as_seconds();
        }

        Ok(Duration { string: string.to_string(), seconds })
    }
}
