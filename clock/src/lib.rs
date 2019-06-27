const FULL_HOUR: i32 = 24;
const FULL_MINUTES: i32 = 60;
const NEG_MINS_AJUSTMENT: i32 = -1;


#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hour = ajust_hours(hours, &minutes);
        let minute = ajust_minutes(minutes);

        Clock{hours: hour % FULL_HOUR, minutes: minute % FULL_MINUTES}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

fn ajust_minutes(minutes: i32) -> i32 {
    match minutes {
        m if minutes >= 0 => m,
        x if minutes < 0 => FULL_MINUTES + (x % FULL_MINUTES),
        _ => 0
    }
}

fn ajust_hours(hours: i32, minutes: &i32) -> i32 {
    let extra_min = minutes / FULL_MINUTES;

    match hours {
            hr if hours >= 0 && minutes < &0 && minutes >= &-60
              => hr + NEG_MINS_AJUSTMENT,
            hr if hours >= 0 && minutes < &0 
              => hr + (FULL_HOUR + (extra_min % FULL_HOUR) + NEG_MINS_AJUSTMENT),
            h if hours >= 0 => h + extra_min,
            xr if hours < 0 && minutes < &0 
              => FULL_HOUR + ((xr + extra_min) % FULL_HOUR) + NEG_MINS_AJUSTMENT,
            x if hours < 0 
              => FULL_HOUR + (x % FULL_HOUR),

            _ => 0
        }
}
