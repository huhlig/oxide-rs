use std::time::Instant;

///
pub trait TimeStep {
    fn step(&self, wall_time: f64) -> f64;
}

/// Keep track of 3 times,
/// RealTime, FrameTime, & PhysicsTime
pub struct Clock {
    real_start_time: Instant,

    frame_count: u64
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            real_start_time: Instant::now(),
            frame_count: 0,
        }
    }
    pub fn tick(){}
    pub fn render()
}