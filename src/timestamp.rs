use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct TimeStamp
{
    pub hours : i64,
    pub minutes : i64,
    pub seconds : i64,
    pub milliseconds : i64
}

const MILLISECONDS_PER_SECOND : i64 = 1000;
const SECONDS_PER_MINUTE : i64 = 60;
const MINUTES_PER_HOUR : i64 = 60;

pub trait TimeStampOps {
    fn subtract(self : &mut Self, st : &TimeStamp);
    fn from_string(string: &str) -> TimeStamp;
    fn from_float(offset: f64) -> TimeStamp;
}

impl TimeStampOps for TimeStamp {
    fn subtract(self: &mut Self, st : &TimeStamp ) {

        self.milliseconds -= st.milliseconds;
        if  self.milliseconds < 0  { 
            self.milliseconds  = MILLISECONDS_PER_SECOND + self.milliseconds;
            self.seconds      -= 1;
        }

        self.seconds -= st.seconds;
        if self.seconds < 0 {
            self.seconds  = SECONDS_PER_MINUTE + self.seconds;
            self.minutes -= 1;
        }

        self.minutes -= st.minutes;
        if self.minutes < 0 {
            self.minutes  = MINUTES_PER_HOUR + self.minutes;
            self.hours   -= 1;
        }

        self.hours -= st.hours;
        if self.hours < 0 {
            panic!("Subtracted timestamp resulted in a negative timestamp");
        }
    }

    fn from_string(string: &str) -> TimeStamp {
        let fields : Vec<&str> = string.split(":").collect();

        let mut result : TimeStamp = TimeStamp::default();

        let get_field = |fields: &Vec<&str>, i: usize| {
            fields.get(i).unwrap().trim().parse::<i64>().unwrap()
        };

        result.hours   = get_field(&fields, 0);
        result.minutes = get_field(&fields, 1);

        // Yes, actively shadowing the first fields variable
        let fields : Vec<&str> = fields.get(2).unwrap().split(",").collect();

        result.seconds      = get_field(&fields, 0);
        result.milliseconds = get_field(&fields, 1);

        result
    }

    fn from_float(offset : f64) -> TimeStamp {

        let seconds = offset.floor();
        let miliseconds = (offset - seconds) * 1000.0;

        TimeStamp { 
            hours : 0, 
            minutes : 0,
            seconds : seconds as i64,
            milliseconds : miliseconds as i64 
        }
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:02}:{:02}:{:02},{:03}",
               self.hours,
               self.minutes,
               self.seconds,
               self.milliseconds
       )
    }
}
