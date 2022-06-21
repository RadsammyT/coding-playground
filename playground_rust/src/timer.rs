use std::time::SystemTime;
use std;

pub(crate) struct Timer {
    pub(crate) start: Option<SystemTime>,
    pub(crate) end: Option<SystemTime>,
}

pub(crate) fn start_timer(timer: &mut Timer) {
    timer.start = Some(SystemTime::now());
}

pub(crate)  fn end_timer(timer: &mut Timer) {
    timer.end = Some(SystemTime::now());
}

pub(crate)  fn get_elapsed(timer: &mut Timer) -> Option<f64> {
    return Some(timer.end?.duration_since(timer.start?).unwrap().as_secs_f64())
}

pub(crate) fn get_epoch(timer: &mut Timer) -> Option<f64> {
    return Some(timer.end?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64());
}


pub(crate)  fn test(exit: bool) {
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

