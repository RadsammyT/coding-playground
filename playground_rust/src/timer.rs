use std::time::SystemTime;


pub(crate) struct timer {
    pub(crate) start: Option<SystemTime>,
    pub(crate) end: Option<SystemTime>,
}

pub(crate) fn start_timer(timer: &mut timer) {
    timer.start = Some(SystemTime::now());
}

pub(crate)  fn end_timer(timer: &mut timer) {
    timer.end = Some(SystemTime::now());
}

pub(crate)  fn get_elapsed(timer: &mut timer) -> Option<f64> {
    return Some(timer.end?.duration_since(timer.start?).unwrap().as_secs_f64())
}
/* 
pub(crate)  fn main() {
    let mut timer = timer {
        start: 0.0,
        end: 0.0,
    };
    
}
*/
