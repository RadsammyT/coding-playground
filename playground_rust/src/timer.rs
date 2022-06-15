use std::time::SystemTime;


pub(crate) struct timer {
    pub(crate) start: f64,
    pub(crate) end: f64,
}

pub(crate) fn start_timer(timer: &mut timer) {
    timer.start = SystemTime::now().elapsed().unwrap().as_secs_f64();
}

pub(crate)  fn end_timer(timer: &mut timer) {
    timer.end = SystemTime::now().elapsed().unwrap().as_secs_f64();
}

pub(crate)  fn get_elapsed(timer: &mut timer) -> f64 {
    return timer.start - timer.end;
    // timer.start - timer.end
}

pub(crate)  fn main() {
    let mut timer = timer {
        start: 0.0,
        end: 0.0,
    };
    
}