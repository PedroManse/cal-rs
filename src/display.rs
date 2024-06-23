use std::fmt;
use crate::date;

impl fmt::Display for date::Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl fmt::Display for date::Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            date::Month::Jan => "Jan",
            date::Month::Feb => "Feb",
            date::Month::Mar => "Mar",
            date::Month::Apr => "Apr",
            date::Month::May => "May",
            date::Month::Jun => "Jun",
            date::Month::Jul => "Jul",
            date::Month::Aug => "Aug",
            date::Month::Sep => "Sep",
            date::Month::Oct => "Oct",
            date::Month::Nov => "Nov",
            date::Month::Dec => "Dec",
        })
    }
}

impl fmt::Display for date::Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            date::Weekday::Sun => "sun",
            date::Weekday::Mon => "mon",
            date::Weekday::Tue => "tue",
            date::Weekday::Wed => "wed",
            date::Weekday::Thu => "thu",
            date::Weekday::Fri => "fri",
            date::Weekday::Sat => "sat",
        })
    }
}

//     Junho 2024     
//dom seg ter qua qui sex sáb
//                          1
//  2   3   4   5   6   7   8
//  9  10  11  12  13  14  15
// 16  17  18  19  20  21  22
// 23  24  25  26  27  28  29
// 30                  

impl fmt::Display for date::Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "")
    }
}
