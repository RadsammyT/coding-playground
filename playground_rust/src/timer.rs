struct timer {
    start: u64,
    end: u64,
}

fn startTimer(timer: &mut timer) {
    timer.start = SystemTime::now().elapsed().unwrap().as_millis();
}

fn endTimer(timer: &mut timer) {
    timer.end = SystemTime::now().elapsed().unwrap().as_millis();
}

fn getElapsed(timer: &mut timer) -> u64 {
    return timer.end - timer.start;
}

fn main() {
    let mut timer = timer {
        start: 0,
        end: 0,
    };
    
}