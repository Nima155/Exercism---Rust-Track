use core::fmt;
#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn hour_handler(time_handle: i32, is_hour: bool) -> i32 {
        println!("{}", time_handle % 60);
        match is_hour {
            true => {
                if time_handle < 0 {
                    return 24 + time_handle;
                } else {
                    return time_handle;
                }
            }
            false => {
                if time_handle < 0 {
                    return 60 + time_handle % 60;
                } else {
                    time_handle % 60
                }
            }
        }
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minute_to_hours = minutes / 60;

        let mut hours_to_hours = (hours + minute_to_hours) % 24;
        if (minutes % 60) > -60 && (minutes % 60) < 0 {
            hours_to_hours -= 1;
        }
        Clock {
            hours: Clock::hour_handler(hours_to_hours, true),
            minutes: if minutes % 60 != 0 {
                Clock::hour_handler(minutes, false)
            } else {
                0
            },
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    // implementing the display trait for clock
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
