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

use super::data::Data;
use super::event::Event;
use super::engine::Engine;
use super::state::{State, StateManager};
use std::time::Instant;

///
///
///
pub struct Application<'a, D: Data> {
    states: StateManager<'a, D>,
    engine: Engine<D>,
}

impl<'a, D: Data> Application<'a, D> {
    pub fn new<S: State<D> + 'a>(initial_state: S, data: D) -> Application<'a, D> {
        let states = StateManager::new(initial_state);
        let engine = Engine::new(data);
        Application { states, engine }
    }
    pub fn run(&mut self) {
        let mut last_update = Instant::now();
        let mut accumulator = 0.0;
        self.states.start(&mut self.engine);

        while self.states.active() {
            let frame_time = self.engine.get_frame_time();
            while accumulator < frame_time {
                let new_time = Instant::now();
                let delta = {
                    let duration = new_time.duration_since(last_update);
                    duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1_000_000_000.0)
                };
                accumulator += delta;
                last_update = new_time;
                // Handle Events Here
                self.states.handle(&mut self.engine, Event::Empty);
                self.states.update(&mut self.engine, delta);
            }

            self.states.render(&mut self.engine);
            accumulator -= frame_time;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Transition;

    struct EmptyData;

    impl Data for EmptyData{}

    struct State1(u64);

    impl State<EmptyData> for State1 {
        fn initialize(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State0 Initialized");
        }
        fn cleanup(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State0 Cleaned up");
        }
        fn suspend(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State0 Suspended");
        }
        fn resume(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State0 Resumed");
        }
        fn handle(&mut self, _engine: &mut Engine<EmptyData>, event: Event) -> Transition<EmptyData> {
            println!("State0 Handled: {:?}", event);
            Transition::Continue
        }
        fn update(&mut self, _engine: &mut Engine<EmptyData>, delta: f64) -> Transition<EmptyData> {
            println!("State0 Updated: {:?} sec delta", delta);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else {
                Transition::Pop
            }
        }
        fn render(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State0 Rendered");
        }
    }


    #[test]
    fn test_runloop() {
        println!("Starting Runloop Test");
        let mut sm = Application::new(State1(25), EmptyData);
        sm.run();
    }
}