use std::fmt;
use chrono::naive::NaiveTime;
use chrono::Duration;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours: hours, minutes: minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let min = minutes as i64;
        let h: u32 = self.hours as u32;
        let m: u32 = self.minutes as u32; 
        let tim = NaiveTime::from_hms(h, m, 0) + Duration::minutes(min);
        println!("{}", tim.format("%H:%M").to_string());

        let rh: i32 = tim.format("%H").to_string().parse::<i32>().unwrap();
        let rm: i32 = tim.format("%M").to_string().parse::<i32>().unwrap();
        Clock { hours: rh, minutes: rm, }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nh: u32 = self.hours as u32;
        let mut dh = Duration::hours(0);
        if self.hours < 0 || self.hours >= 24 {
            nh = 0 as u32;
            let xh: i64 = self.hours as i64;
            dh = Duration::hours(xh);
        }

        let mut nm: u32 = self.minutes as u32;
        let mut dm = Duration::minutes(0);
        if self.minutes < 0 || self.minutes >= 60 {
            nm = 0 as u32;
            let xm: i64 = self.minutes as i64;
            dm = Duration::minutes(xm);
        }

        let tim = NaiveTime::from_hms(nh, nm, 0) + dh + dm;
        let h: i32 = tim.format("%H").to_string().parse::<i32>().unwrap();
        let m: i32 = tim.format("%M").to_string().parse::<i32>().unwrap();
        write!(f, "{:0>2}:{:0>2}", h, m)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let mut nh: u32 = self.hours as u32;
        let mut dh = Duration::hours(0);
        if self.hours < 0 || self.hours > 23 {
            nh = 0 as u32;
            let xh: i64 = self.hours as i64;
            dh = Duration::hours(xh);
        }

        let mut nm: u32 = self.minutes as u32;
        let mut dm = Duration::minutes(0);
        if self.minutes < 0 || self.minutes >= 60 {
            nm = 0 as u32;
            let xm: i64 = self.minutes as i64;
            dm = Duration::minutes(xm);
        }

        let tim = NaiveTime::from_hms(nh, nm, 0) + dh + dm;
        let h: i32 = tim.format("%H").to_string().parse::<i32>().unwrap();
        let m: i32 = tim.format("%M").to_string().parse::<i32>().unwrap();

        // others
        let mut o_nh: u32 = other.hours as u32;
        let mut o_dh = Duration::hours(0);
        if other.hours < 0 || other.hours > 23 {
            o_nh = 0 as u32;
            let o_xh: i64 = other.hours as i64;
            o_dh = Duration::hours(o_xh);
        };

        let mut o_nm: u32 = other.minutes as u32;
        let mut o_dm = Duration::minutes(0);
        if other.minutes < 0 || other.minutes >= 60 {
            o_nm = 0 as u32;
            let o_xm: i64 = other.minutes as i64;
            o_dm = Duration::minutes(o_xm);
        }

        let o_tim = NaiveTime::from_hms(o_nh, o_nm, 0) + o_dh + o_dm;
        let o_h: i32 = o_tim.format("%H").to_string().parse::<i32>().unwrap();
        let o_m: i32 = o_tim.format("%M").to_string().parse::<i32>().unwrap();

        if h == o_h && m == o_m {
            true
        } else {
            false
        }
    }
}