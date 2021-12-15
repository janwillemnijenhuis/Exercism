use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::{PrimitiveDateTime as DateTime, PrimitiveDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start.add(Duration::from_secs(1000000000));

}

fn main() {
    after(dt(2011, 4, 25, 0, 0, 0));
}

fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};

    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}