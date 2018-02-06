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
//! State Engine
//!

use ::event::Event;

/// State Transition commands
pub enum Transition {
    /// No Transition
    Continue,
    /// Call Cleanup on current state, pop it off the stack, call resume on the next state if it
    /// exist.
    Pop,
    /// Call Suspend the current state, push a new State onto the stack, call initialize.
    Push(Box<State>),
    /// Call cleanup on the current state, pop it off the stackSwitch to a new State by Popping off the old one and Pushing on a new one.
    Switch(Box<State>),
    /// Pop All States and shut down.
    Halt,
}

///
/// Engine State
///
pub trait State {
    /// Called Once on State creation.
    fn initialize(&mut self);
    /// Called Once before State destruction.
    fn cleanup(&mut self);
    /// Called if State is suspended.
    fn suspend(&mut self);
    /// Called when state returns from suspension.
    fn resume(&mut self);
    /// Handle Events
    fn handle(&mut self, event: Event) -> Transition;
    /// Called Periodically during updates.
    fn update(&mut self, delta: f64) -> Transition;
    /// Render State to Screen.
    fn render(&mut self);
}

/// Simple Stack based State Machine
pub(crate) struct StateMachine<'a> {
    states: Vec<Box<State + 'a>>,
    active: bool,
}

impl<'a> StateMachine<'a> {
    /// Create a new State Machine with an Initial State
    pub(crate) fn new<S: State + 'a>(initial_state: S) -> StateMachine<'a> {
        StateMachine {
            states: vec![Box::new(initial_state)],
            active: false,
        }
    }
    pub(crate) fn active(&self) -> bool {
        self.active
    }
    pub(crate) fn start(&mut self) {
        if !self.active {
            self.states.last_mut().unwrap().initialize();
            self.active = true;
        }
    }
    pub(crate) fn handle(&mut self, event: Event) {
        if self.active {
            let transition = match self.states.last_mut() {
                Some(state) => state.handle(event),
                None => Transition::Continue,
            };
            self.transition(transition);
        }
    }
    pub(crate) fn update(&mut self, delta: f64) {
        if self.active {
            let transition = match self.states.last_mut() {
                Some(state) => state.update(delta),
                None => Transition::Continue,
            };
            self.transition(transition);
        }
    }
    pub(crate) fn render(&mut self) {
        if self.active {
            match self.states.last_mut() {
                Some(state) => state.render(),
                None => Transition::Continue,
            };
        }
    }
    pub(crate) fn stop(&mut self) {
        if self.active {
            while let Some(mut state) = self.states.pop() {
                state.cleanup();
            }
            self.active = false;
        }
    }
    fn transition(&mut self, transition: Transition) {
        if self.active {
            match transition {
                Transition::Continue => (),
                Transition::Pop => self.pop(),
                Transition::Push(state) => self.push(state),
                Transition::Switch(state) => self.switch(state),
                Transition::Halt => self.stop(),
            }
        }
    }
    fn push(&mut self, state: Box<State>) {
        if self.active {
            // Suspend currently active state.
            if let Some(state) = self.states.last_mut() {
                state.suspend();
            }

            self.states.push(state);
            let state = self.states.last_mut().unwrap();
            state.initialize();
        }
    }
    fn pop(&mut self) {
        if self.active {
            if let Some(mut state) = self.states.pop() {
                state.cleanup();
            }

            if let Some(state) = self.states.last_mut() {
                state.resume();
            } else {
                self.active = false;
            }
        }
    }
    fn switch(&mut self, state: Box<State>) {
        if self.active {
            if let Some(mut state) = self.states.pop() {
                state.cleanup();
            }

            self.states.push(state);
            let state = self.states.last_mut().unwrap();
            state.initialize();
        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    struct State1(u8, u8);

    struct State2(u8);

    struct State3(u8);

    impl State for State1 {
        fn initialize(&mut self) {
            println!("State1 Initialized");
        }
        fn cleanup(&self) {
            println!("State1 Cleaned up");
        }
        fn suspend(&mut self) {
            println!("State1 Suspended");
        }
        fn resume(&mut self) {
            println!("State1 Resumed");
        }
        fn handle(&mut self, event: Event) -> Transition {
            println!("State1 Handled {:?}", event);
        }
        fn update(&mut self, delta: f64) -> Transition {
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
        fn render(&self) {
            println!("State1 Rendered");
        }
    }

    impl State for State2 {
        fn initialize(&mut self) {
            println!("State2 Initialized");
        }
        fn cleanup(&self) {
            println!("State2 Cleaned up");
        }
        fn suspend(&mut self) {
            println!("State2 Suspended");
        }
        fn resume(&mut self) {
            println!("State2 Resumed");
        }
        fn handle(&mut self, event: Event) -> Transition {
            println!("State2 Handled {:?}", event);
        }
        fn update(&mut self, delta: f64) -> Transition {
            println!("State2 Updated {:?}", event);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else {
                Transition::Pop
            }
        }
        fn render(&self) {
            println!("State2 Rendered");
        }
    }

    impl State for State3 {
        fn initialize(&mut self) { println!("State3 Initialized"); }
        fn cleanup(&self) {
            println!("State3 Cleaned up");
        }
        fn suspend(&mut self) {
            println!("State3 Suspended");
        }
        fn resume(&mut self) {
            println!("State3 Resumed");
        }
        fn handle(&mut self, event: Event) -> Transition {
            println!("State3 Handled {:?}", event);
        }
        fn update(&mut self, delta: f64) -> Transition {
            println!("State3 Updated {:?}", event);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else {
                Transition::Pop
            }
        }
        fn render(&self) {
            println!("State3 Rendered");
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