use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new1(mut hours: i32, mut minutes: i32) -> Self {
        hours = hours + minutes / 60;
        minutes = minutes % 60;
        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        hours = hours % 24;
        if hours < 0 {
            hours += 24;
        }
        Self { hours, minutes }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: minutes.rem_euclid(60),
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
