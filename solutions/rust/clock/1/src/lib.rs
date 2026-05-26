use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Self { hours, minutes: 0 };
        clock.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut minutes = self.minutes + minutes;
        let mut hours = self.hours + minutes / 60;
        minutes %= 60;
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
            minutes %= 60;
        }
        hours %= 24;
        if hours < 0 {
            hours += 24;
            hours %= 24;
        }
        Self { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
