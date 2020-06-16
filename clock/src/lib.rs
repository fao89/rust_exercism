use std::fmt;

const HOUR_IN_MINUTES: i32 = 60;
const DAY_IN_MINUTES: i32 = HOUR_IN_MINUTES * 24;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = Self::to_minutes(hours, minutes);
        Self {
            hours: total_minutes / HOUR_IN_MINUTES,
            minutes: total_minutes % HOUR_IN_MINUTES,
        }
    }

    pub fn to_minutes(hours: i32, minutes: i32) -> i32 {
        let total_minutes = (hours * HOUR_IN_MINUTES + minutes) % DAY_IN_MINUTES;
        (total_minutes + DAY_IN_MINUTES) % DAY_IN_MINUTES
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = Self::to_minutes(self.hours, self.minutes + minutes);
        Self {
            hours: total_minutes / HOUR_IN_MINUTES,
            minutes: total_minutes % HOUR_IN_MINUTES,
        }
    }
}
