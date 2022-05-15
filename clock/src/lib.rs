use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut extra_hours = minutes / 60;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes = 60 + minutes;
            extra_hours -= 1;
        }
        if extra_hours < 0 {
            extra_hours = 24 + extra_hours % 24;
        }
        let mut hours = if hours < 0 { 24 + hours % 24 } else { hours % 24 };
        hours = (hours + extra_hours) % 24;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let val = Self::new(0, minutes);
        Self::new(self.hours + val.hours, val.minutes + self.minutes)
    }
}
