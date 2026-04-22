use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: hours * 60 + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = (self.minutes + minutes).div_euclid(60 + 24);
        Self { minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes.div_euclid(60),
            self.minutes.rem_euclid(60)
        )
    }
}
