use std::time::SystemTime;
use std;

pub struct Timer {
    pub start: Option<SystemTime>,
    pub end: Option<SystemTime>,
}

impl Timer {
    ///returns a timer struct with both SystemTimes set to None
    /// 
    /// # Example
    /// ```
    /// let mut timer = Timer::new();
    /// ```
    pub fn new() -> Timer {
       return Timer {start: None, end: None}
    }

    pub fn start_timer(&mut self) {
        self.start = Some(SystemTime::now());
    }

    pub fn end_timer(&mut self) {
        if self.start != None {
            self.end = Some(SystemTime::now());
        } else {
            panic!("Invocation of end_timer when start has not been set");
        }
    }

    pub fn get_elapsed(&mut self) -> Option<f64> {
        return Some(self.end?.duration_since(self.start?).unwrap().as_secs_f64())
    }
    
    ///Gets the epoch time from either `timer.start` or `timer.end`.
    /// set `time` argument to true to get EPOCH from `timer.start`
    /// set to false to get EPOCH from `timer.false`
    /// # Example
    /// 
    /// 
    /// ```
    /// let mut timer = Timer::new();
    /// Timer::start_timer(&mut timer);
    /// 
    /// println!("{}", Timer::get_epoch(&mut timer, true).unwrap());
    /// ```
    #[allow(dead_code)]
    pub fn get_epoch(&mut self, time: bool) -> Option<f64> {
        if time {
            return Some(self.start?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
        } else {
            return Some(self.end?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
        }
    }

}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer").field("start", &self.start).field("end", &self.end).finish()
    }
}

#[allow(dead_code)]
pub fn test(exit: bool) {
    let mut timer = Timer {
        start: None,
        end: None,
    };
    Timer::start_timer(&mut timer);
    println!("{}", Timer::get_epoch(&mut timer, true).unwrap());

    if exit {
        println!("exiting");
        std::process::exit(0);
    }
    
}

