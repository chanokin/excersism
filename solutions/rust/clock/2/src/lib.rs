#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hrs = hours;
        
        if hrs < 0 {
            hrs = 24 + (hrs % 24);
        }

        Self {
            hours: hrs,
            minutes: 0
        }.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes = self.minutes + minutes;
        let mut hr_diff = new_minutes / 60;

        if new_minutes < 0 && minutes != -60{
            hr_diff -= 1;
        }
        
        new_minutes %= 60;
        
        if new_minutes < 0 {
            new_minutes += 60;
        }

        let mut new_hours = (self.hours + hr_diff) % 24;

        if new_hours < 0 {
            new_hours += 24;
        }
        
        Self{
            hours: new_hours,
            minutes: new_minutes
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}