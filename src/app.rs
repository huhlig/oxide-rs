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

use super::services::Engine;
use super::event::Event;
use super::state::{State, StateManager};
use std::any::Any;
use std::convert::From;
use std::time::Instant;

///
///
///
pub struct Application<'a, D>
    where
        D: Any + Send + Sync
{
    states: StateManager<'a>,
    engine: Engine,
    frame_time: f64,
    data: D,
}

impl<'a, D> Application<'a, D> {
    pub(crate) fn create<S: State + 'a>(initial_state: S, data: D) -> Application<'a, D> {
        let states = StateManager::new(initial_state);
        let engine = Engine {};
        let frame_time = 1.0 / 30.0;
        Application { states, engine, frame_time, data }
    }
    pub fn run(&self) {
        let mut t = 0.0;
        let mut last_update = Instant::now();
        let mut accumulator = 0.0;

        loop {

            while accumulator < self.frame_time {
                let new_time = Instant::now();
                let mut delta = Instant::from(new_time, current_time).as_nanos / 1_000_000_000; // from ns to s
                accumulator += frame_time;
                current_time = new_time;
                // Handle Events Here
                //self.states.handle(Event::Empty);
                //self.states.update(dt);
                accumulator += dt;
                t += dt;
            }

            self.states.render(&mut self.engine);

        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    struct State1;

    impl State for State1 {
        fn initialize(&mut self, engine: &mut Engine) {
            println!("State1 Initialized");
        }
        fn cleanup(&mut self, engine: &mut Engine) {
            println!("State1 Cleaned up");
        }
        fn suspend(&mut self, engine: &mut Engine) {
            println!("State1 Suspended");
        }
        fn resume(&mut self, engine: &mut Engine) {
            println!("State1 Resumed");
        }
        fn handle(&mut self, engine: &mut Engine, event: Event) -> Transition {
            println!("State1 Handled {:?}", event);
        }
        fn update(&mut self, engine: &mut Engine, delta: f64) -> Transition {
            println!("State1 Updated {:?}", event);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else if self.1 > 0 {
                self.1 -= 1;
                Transition::Push(Box::new(State2(10)))
            } else {
                Transition::Switch(Box::new(State3(10)))
            }
        }
        fn render(&mut self, engine: &mut Engine) {
            println!("State1 Rendered");
        }
    }


    #[test]
    fn switch_pop() {
        let mut sm = StateMachine::new(State1(10, 5));
        sm.start();

        for _ in 0..8 {
            for _ in 0..4 {
                sm.update();
            }
            sm.render();
            assert!(sm.active());
        }

        assert!(!sm.active());
    }
}