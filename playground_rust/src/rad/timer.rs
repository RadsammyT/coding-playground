use std::time::SystemTime;
use std;

pub struct Timer {
    pub start: Option<SystemTime>,
    pub end: Option<SystemTime>,
}

pub fn start_timer(timer: &mut Timer) {
    timer.start = Some(SystemTime::now());
}

pub  fn end_timer(timer: &mut Timer) {
    timer.end = Some(SystemTime::now());
}

pub  fn get_elapsed(timer: &mut Timer) -> Option<f64> {
    return Some(timer.end?.duration_since(timer.start?).unwrap().as_secs_f64())
}

pub fn get_epoch(timer: &mut Timer) -> Option<f64> {
    return Some(timer.end?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
}


pub  fn test(exit: bool) {
    let mut timer = Timer {
        start: None,
        end: None,
    };
    start_timer(&mut timer);
    end_timer(&mut timer);
    println!("{}", get_epoch(&mut timer).unwrap());

    if exit {
        println!("exiting");
        std::process::exit(0);
    }
    
}

