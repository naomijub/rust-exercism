#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hour = match hours {
            h if hours >= 0 => h,
            x if hours < 0 => 24 + x,
            _ => 0
        };

        let extra_min = minutes / 60;
        Clock{hours: (hour + extra_min) % 24, minutes: minutes % 60}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
