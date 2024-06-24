use crate::error::CalError;
#[derive(Debug)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[derive(Debug)]
pub enum Weekday {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

#[derive(Debug)]
pub struct Year(pub i32);

impl Year {
    pub fn is_leap(&self) -> bool {
        (self.0 % 4) == 0 && (self.0 % 100 != 00 || self.0 % 400 == 0)
    }
}

impl TryFrom<u32> for Month {
    type Error = CalError;
    fn try_from(u: u32) -> Result<Month, CalError> {
        Ok(match u {
            1 => Month::Jan,
            2 => Month::Feb,
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            x => Err(CalError::MonthRange(x))?,
        })
    }
}

impl Month {
    pub fn next(&self) -> Month {
        match self {
            Month::Dec => Month::Jan,
            Month::Jan => Month::Feb,
            Month::Feb => Month::Mar,
            Month::Mar => Month::Apr,
            Month::Apr => Month::May,
            Month::May => Month::Jun,
            Month::Jun => Month::Jul,
            Month::Jul => Month::Aug,
            Month::Aug => Month::Sep,
            Month::Sep => Month::Oct,
            Month::Oct => Month::Nov,
            Month::Nov => Month::Dec,
        }
    }
    pub fn prev(&self) -> Month {
        match self {
            Month::Feb => Month::Jan,
            Month::Mar => Month::Feb,
            Month::Apr => Month::Mar,
            Month::May => Month::Apr,
            Month::Jun => Month::May,
            Month::Jul => Month::Jun,
            Month::Aug => Month::Jul,
            Month::Sep => Month::Aug,
            Month::Oct => Month::Sep,
            Month::Nov => Month::Oct,
            Month::Dec => Month::Nov,
            Month::Jan => Month::Dec,
        }
    }
    pub fn day_count(&self, year: &Year) -> u32 {
        match self {
            Month::Jan => 31,
            Month::Feb => 28 + if year.is_leap() { 1 } else { 0 },
            Month::Mar => 31,
            Month::Apr => 30,
            Month::May => 31,
            Month::Jun => 30,
            Month::Jul => 31,
            Month::Aug => 31,
            Month::Sep => 30,
            Month::Oct => 31,
            Month::Nov => 30,
            Month::Dec => 31,
        }
    }
}

#[derive(Debug)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: u32,
    pub date: NaiveDate,
    pub first_of_month: NaiveDate,
}

use chrono::{Datelike, Local, NaiveDate};
impl Date {
    pub fn gather_important(&self) -> Vec<(u32, &str)> {
        vec![
            (3, "dia 3"),
            (5, "dia 5"),
            (6, "dia 6!!"),
        ]
    }
    pub fn now() -> Result<Date, CalError> {
        let date = Local::now().date_naive();
        let year = Year(date.year());
        let month = date.month().try_into()?;
        let day = date.day();
        let first = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
            .ok_or(CalError::FirstDayError)?;
        Ok(Date {
            year,
            month,
            day,
            date,
            first_of_month: first,
        })
    }
}
