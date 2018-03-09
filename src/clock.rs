//
// Copyright 2017 Hans W. Uhlig.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::time::{Duration, Instant};

const NANOS_PER_SECOND: f64 = 1_000_000_000;

/// Keep track of 3 times,
/// RealTime, FrameTime, & PhysicsTime
pub struct Clock {
    real_start_time: Instant,
    last_update: Instant,
    last_frame: Instant,
    frame_count: u64,
}

impl Clock {
    pub fn new() -> Clock {
        let time = Instant::new();
        Clock {
            real_start_time: time,
            last_update: time,
            last_frame: time,
            frame_count: 0,
        }
    }
    pub fn update_tick(&mut self) -> f64 {
        let new_time = Instant::now();
        let delta_time = new_time.duration_since(self.last_update);
        self.last_update = new_time;
        
    }
    pub fn render_tick() -> f64 {}
}