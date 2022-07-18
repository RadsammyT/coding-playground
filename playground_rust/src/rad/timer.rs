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
            eprintln!("RAD/TIMER ERROR: TIMER.START IS NONE. {:?}", self);
        }
    }

    pub fn get_elapsed(&mut self) -> Option<f64> {
        return Some(self.end?.duration_since(self.start?).unwrap().as_secs_f64())
    }
    
    ///Gets the epoch time from ```timer.start```.
    /// # Example
    /// 
    /// 
    /// ```
    /// let mut timer = Timer::new();
    /// Timer::start_timer(&mut timer);
    /// 
    /// println!("{}", Timer::get_epoch(&mut timer).unwrap());
    /// ```
    pub fn get_epoch(&mut self) -> Option<f64> {
        return Some(self.start?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
    }

}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer").field("start", &self.start).field("end", &self.end).finish()
    }
}


pub fn test(exit: bool) {
    let mut timer = Timer {
        start: None,
        end: None,
    };
    Timer::start_timer(&mut timer);
    Timer::end_timer(&mut timer);
    println!("{}", Timer::get_epoch(&mut timer).unwrap());

    if exit {
        println!("exiting");
        std::process::exit(0);
    }
    
}

