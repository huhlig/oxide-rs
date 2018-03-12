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
//!
//! Application Framework
//!

use super::event::Event;
use super::engine::Engine;
use super::state::{State, StateManager};
use std::any::Any;
use std::time::Instant;

///
///
///
pub struct Application<'a, D>
    where
        D: Any + Send + Sync
{
    states: StateManager<'a>,
    frame_time: f64,
    data: D,
}

impl<'a, D> Application<'a, D>
    where
        D: Any + Send + Sync
{
    pub(crate) fn create<S: State + 'a>(initial_state: S, data: D) -> Application<'a, D> {
        let states = StateManager::new(initial_state);
        let frame_time = 1.0 / 30.0;
        Application { states, frame_time, data }
    }
    pub fn run(&mut self) {
        let mut last_update = Instant::now();
        let mut accumulator = 0.0;

        loop {
            while accumulator < self.frame_time {
                let new_time = Instant::now();
                let delta = {
                    let duration = new_time.duration_since(last_update);
                    duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1_000_000_000.0)
                };
                accumulator += delta;
                last_update = new_time;
                // Handle Events Here
                self.states.handle(&mut self, Event::Empty);
                self.states.update(&mut self, delta);
            }

            self.states.render(&mut self);
            accumulator -= self.frame_time;
        }
    }
}

impl<'a, D> Engine for Application<'a,D>
    where
        D: Any + Send + Sync
{

}

#[cfg(tests)]
mod tests {
    use super::*;

    struct State1(u64);

    impl State for State1 {
        fn initialize(&mut self, engine: &mut Engine) {
            println!("State Initialized");
        }
        fn cleanup(&mut self, engine: &mut Engine) {
            println!("State Cleaned up");
        }
        fn suspend(&mut self, engine: &mut Engine) {
            println!("State Suspended");
        }
        fn resume(&mut self, engine: &mut Engine) {
            println!("State Resumed");
        }
        fn handle(&mut self, engine: &mut Engine, event: Event) -> Transition {
            println!("State1 Handled {:?}", event);
        }
        fn update(&mut self, engine: &mut Engine, delta: f64) -> Transition {
            println!("State Updated: {:?} sec delta", delta);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else {
                Transition::Pop
            }
        }
        fn render(&mut self, engine: &mut Engine) {
            println!("State Rendered");
        }
    }

    struct EmptyData;

    #[test]
    fn test_runloop() {
        let mut sm = Application::create(State1(65_535), EmptyData);
        sm.run();
    }
}