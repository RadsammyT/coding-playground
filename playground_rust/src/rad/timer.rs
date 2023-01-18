use std::time::SystemTime;
use std;
pub struct Timer {
    start: Option<SystemTime>,
    end: Option<SystemTime>,
    panic_at_error: bool,
}

impl Default for Timer {
    ///returns a timer struct with both SystemTimes set to None
    /// 
    /// # Example
    /// ```
    /// let mut timer = Timer::new();
    /// ```
    fn default() -> Self {
        Self {
            start: None, end: None, panic_at_error: true,
        }
    }
}

impl Timer {

    /// 
    /// returns a timer struct with both SystemTimes set to None
    /// 
    /// # Example
    /// ```
    /// let mut timer = Timer::new();
    /// ```
    /// Alternatively, you can use Timer::default()
    pub fn new() -> Timer {
        return Timer {start: None, end: None, panic_at_error: true}
    }

    pub fn start_timer(&mut self) {
        self.start = Some(SystemTime::now());

        match self.end {
            Some(_) => {
                self.end = None;
            },
            None => {

            }
        }
    }

    pub fn end_timer(&mut self) {

        match self.end {
            Some(_) => {
                if self.panic_at_error { panic!("Invocation of end_timer when it has already been set"); }
            },
            None => {
                self.end = Some(SystemTime::now());
            }
        }

        match self.start {
            Some(_) => {
                self.end = Some(SystemTime::now());
            },
            None => {
                if self.panic_at_error { panic!("Invocation of end_timer when start has not been set"); }
            }
        }
    }

    /// ```end_timer``` but one word is switched.
    /// I thought it might be a little easier on some people (myself included),
    /// so I added this method as a convenience
    /// 
    pub fn stop_timer(&mut self) {
        self.end_timer();
    }

    pub fn get_elapsed(&mut self) -> Option<f64> {
        return Some(self.end?.duration_since(self.start?).unwrap().as_secs_f64())
    }
    
    ///Gets the epoch time from either `timer.start` or `timer.end`.
    /// * set `time` parameter to true to get EPOCH from `timer.start`
    /// * set to false to get EPOCH from `timer.end`
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
        panic_at_error: true,
    };
    Timer::start_timer(&mut timer);
    println!("{}", Timer::get_epoch(&mut timer, true).unwrap());

    if exit {
        println!("exiting");
        std::process::exit(0);
    }
    
}
