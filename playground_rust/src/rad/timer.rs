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


    pub fn start_timer(timer: &mut Timer) {
        timer.start = Some(SystemTime::now());
    }

    pub fn end_timer(timer: &mut Timer) {
        timer.end = Some(SystemTime::now());
    }

    pub fn get_elapsed(timer: &mut Timer) -> Option<f64> {
        return Some(timer.end?.duration_since(timer.start?).unwrap().as_secs_f64())
    }

    pub fn get_epoch(timer: &mut Timer) -> Option<f64> {
        return Some(timer.end?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
    }

}
pub  fn test(exit: bool) {
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

