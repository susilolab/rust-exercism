use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = 0;
        let mut m = 0;
        if minutes > 23 {
            m = minutes % 60;
            h = minutes / 60;

        } else {
            m = minutes;
        }

        if hours > 23 {
            h = (h + hours) % 24;
        } else {
            h = h + hours;
        }

        Clock { hours: h, minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total = (self.hours * 60) + self.minutes + minutes;

        let mut h = 0;
        let mut m = 0;
        if total > 0 {
            m = total % 60;
            h = total/60;
        } else {
            m = total % -60;
            h = total / -60;
        };

        Clock { hours: h, minutes: m }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
