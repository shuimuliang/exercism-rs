use std::fmt;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut human_minutes=  minutes;
        let mut human_hours = hours;
        let delta_hours ;

        if minutes < 0 {
            if minutes % 60 == 0 {
                delta_hours = minutes / 60;
                human_minutes = 0;
            } else {
                delta_hours = -1 + minutes / 60;
                human_minutes = minutes % 60 + 60;
            }
        } else {
            delta_hours = human_minutes / 60;
            human_minutes = minutes % 60;
        }

        human_hours += delta_hours;

        if human_hours < 0 {
            if human_hours % 24 == 0 {
                human_hours = 0;
            } else {
                human_hours = human_hours % 24 + 24;
            }
        } else {
            human_hours = human_hours % 24;
        }
        Self {
           hours: human_hours,
           minutes: human_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
        // let forward_min = self.minutes + minutes;
        // let add_hours = forward_min / 60;
        //
        // Self {
        //     hours: (self.hours +  add_hours) % 24,
        //     minutes: forward_min % 60,
        // }
    }
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
