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
//! Event Framework
//!

use ::event::Event;
use ::state::{State, StateMachine};
use std::time::Instant;
use std::convert::From;

///
///
///
pub struct Application<'a> {
    states: StateMachine<'a>,
}

const dt: f64 = 0.01;

impl<'a> Application<'a> {
    pub(crate) fn create<S: State + 'a>(initial_state: S) -> Application<'a> {
        Application {
            states: StateMachine::new(initial_state),
        }
    }
    pub fn run(&self) {
        let mut t = 0.0;
        let mut current_time = Instant::now();
        let mut accumulator = 0.0;

        loop {
            let new_time = Instant::now();
            let mut frame_time = std::time::Instant::from(new_time, current_time).as_nanos / 1_000_000_000; // from ns to s
            current_time = new_time;

            accumulator += frame_time;

            while accumulator >= dt {
                // Handle Events Here
                self.states.handle(Event::Empty);
                self.states.update(dt);
                accumulator -= dt;
                t += dt;
            }

            self.states.render();
        }
    }
}