const FULL_HOUR: i32 = 24;
const FULL_MINUTES: i32 = 60;
const EXTRA_MINS: i32 = -1;


#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_min = minutes / FULL_MINUTES;

        let hour = match hours {
            hr if hours >= 0 && minutes < 0 && minutes >= -60
              => hr + EXTRA_MINS,
            hr if hours >= 0 && minutes < 0 
              => hr + (FULL_HOUR + (extra_min % 24) + EXTRA_MINS),
            h if hours >= 0 => h + extra_min,
            x if hours < 0 => FULL_HOUR + (x % FULL_HOUR),
            _ => 0
        };

        let minute = match minutes {
            m if minutes >= 0 => m,
            x if minutes < 0 => FULL_MINUTES + (x % FULL_MINUTES),
            _ => 0
        };

        Clock{hours: hour % FULL_HOUR, minutes: minute % FULL_MINUTES}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
