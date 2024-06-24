use crate::date;
use std::fmt;

impl ToString for date::Year {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl ToString for date::Month {
    fn to_string(&self) -> String {
        match self {
            date::Month::Jan => "January",
            date::Month::Feb => "February",
            date::Month::Mar => "March",
            date::Month::Apr => "April",
            date::Month::May => "May",
            date::Month::Jun => "June",
            date::Month::Jul => "July",
            date::Month::Aug => "August",
            date::Month::Sep => "September",
            date::Month::Oct => "October",
            date::Month::Nov => "November",
            date::Month::Dec => "December",
        }
        .to_owned()
    }
}

impl ToString for date::Weekday {
    fn to_string(&self) -> String {
        match self {
            date::Weekday::Sun => "sun",
            date::Weekday::Mon => "mon",
            date::Weekday::Tue => "tue",
            date::Weekday::Wed => "wed",
            date::Weekday::Thu => "thu",
            date::Weekday::Fri => "fri",
            date::Weekday::Sat => "sat",
        }
        .to_owned()
    }
}

impl date::Weekday {
    fn strings() -> Vec<String> {
        ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
            .into_iter()
            .map(String::from)
            .collect()
    }
}

//        June 2024
//dom seg ter qua qui sex sáb
// 26  27  28  29  30  31   1
//  2   3   4   5   6   7   8
//  9  10  11  12  13  14  15
// 16  17  18  19  20  21  22
// 23  24  25  26  27  28  29
// 30   1   2   3   4   5   6

use color_print::cformat;
use chrono::Datelike;
impl fmt::Display for date::Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let month = self.month.to_string();
        let padding = " ".repeat((18 - month.len()) / 2);
        let year = self.year.to_string();
        let day = self.day.to_string();
        write!(f, "{padding}{day: >2}, {month} {year}\n")?;
        write!(f, "{}\n", date::Weekday::strings().iter().fold(String::new(), |i, t|i+t+" "))?;
        let prev_month_count = self.month.prev().day_count(&self.year);
        let this_month_count = self.month.day_count(&self.year);
        let prev_appear_count = self.first_of_month.weekday().num_days_from_sunday();
        let mut callendar = String::with_capacity(4*7*5);
        let mut dtot = prev_appear_count;
        for i in 0..prev_appear_count {
            let day = prev_month_count - (prev_appear_count-i);
            callendar.push_str(&cformat!(" <black!>{day: >2}</> "));
        }
        let days = self.gather_important();
        let days_check: Vec<u32> = days.iter().map(|(day, _name)|*day).collect();
        for day in 1..=this_month_count {
            dtot+=1;
            if self.day == day {
                callendar.push_str(&cformat!("<w!>[{day: >2}]</>"));
            } else {
                if days_check.iter().any(|&i|i==day) {
                    callendar.push_str(&cformat!(" <g>{day: >2}</> "));
                } else {
                    callendar.push_str(&cformat!(" <w>{day: >2}</> "));
                }
            }
            if dtot%7==0 {
                callendar.push('\n');
            }
        }
        for day in 1..=7-(dtot%7) {
            callendar.push_str(&cformat!("<black!>{day: >3}</> "));
        }
        write!(f, "{}\n\n", callendar)?;
        days.into_iter().map(|(day, desc)|{
            write!(f, "{day}: {desc}\n")
        }).collect()
    }
}
