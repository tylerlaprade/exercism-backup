use std::fmt;

const DAY: i32 = 60 * 24;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (hours * 60 + minutes).rem_euclid(DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = (self.minutes + minutes).rem_euclid(DAY);
        Self { minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / 60,
            self.minutes.rem_euclid(60)
        )
    }
}
