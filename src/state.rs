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

use super::data::Data;
use super::engine::Engine;
use super::event::Event;

/// State Transition commands
pub enum Transition<D: Data> {
    /// No Transition
    Continue,
    /// Call Cleanup on current state, pop it off the stack, call resume on the next state if it
    /// exist.
    Pop,
    /// Call Suspend the current state, push a new State onto the stack, call initialize.
    Push(Box<State<D>>),
    /// Call cleanup on the current state, pop it off the stackSwitch to a new State by Popping off the old one and Pushing on a new one.
    Switch(Box<State<D>>),
    /// Pop All States and shut down.
    Halt,
}

///
/// Engine State
///
pub trait State<D: Data> {
    /// Called Once on State creation.
    fn initialize(&mut self, engine: &mut Engine<D>);
    /// Called Once before State destruction.
    fn cleanup(&mut self, engine: &mut Engine<D>);
    /// Called if State is suspended.
    fn suspend(&mut self, engine: &mut Engine<D>);
    /// Called when state returns from suspension.
    fn resume(&mut self, engine: &mut Engine<D>);
    /// Handle Events
    fn handle(&mut self, engine: &mut Engine<D>, event: Event) -> Transition<D>;
    /// Called Periodically during updates.
    fn update(&mut self, engine: &mut Engine<D>, delta: f64) -> Transition<D>;
    /// Render State to Screen.
    fn render(&mut self, engine: &mut Engine<D>);
}


/// Simple Stack based State Machine
pub(crate) struct StateManager<'a, D: Data> {
    states: Vec<Box<State<D> + 'a>>,
    active: bool,
}

impl<'a, D: Data> StateManager<'a, D> {
    /// Create a new State Machine with an Initial State
    pub(crate) fn new<S: State<D> + 'a>(initial_state: S) -> StateManager<'a, D> {
        StateManager {
            states: vec![Box::new(initial_state)],
            active: false,
        }
    }
    pub(crate) fn active(&self) -> bool {
        self.active
    }
    pub(crate) fn start(&mut self, engine: &mut Engine<D>) {
        if !self.active {
            self.states.last_mut().unwrap().initialize(engine);
            self.active = true;
        }
    }
    pub(crate) fn handle(&mut self, engine: &mut Engine<D>, event: Event) {
        if self.active {
            let transition = match self.states.last_mut() {
                Some(state) => state.handle(engine, event),
                None => Transition::Continue,
            };
            self.transition(engine, transition);
        }
    }
    pub(crate) fn update(&mut self, engine: &mut Engine<D>, delta: f64) {
        if self.active {
            let transition = match self.states.last_mut() {
                Some(state) => state.update(engine, delta),
                None => Transition::Continue,
            };
            self.transition(engine, transition);
        }
    }
    pub(crate) fn render(&mut self, engine: &mut Engine<D>) {
        if self.active {
            self.states.last_mut().unwrap().render(engine);
        }
    }
    pub(crate) fn stop(&mut self, engine: &mut Engine<D>) {
        if self.active {
            while let Some(mut state) = self.states.pop() {
                state.cleanup(engine);
            }
            self.active = false;
        }
    }
    fn transition(&mut self, engine: &mut Engine<D>, transition: Transition<D>) {
        if self.active {
            match transition {
                Transition::Continue => (),
                Transition::Pop => self.pop(engine),
                Transition::Push(state) => self.push(engine, state),
                Transition::Switch(state) => self.switch(engine, state),
                Transition::Halt => self.stop(engine),
            }
        }
    }
    fn push(&mut self, engine: &mut Engine<D>, state: Box<State<D>>) {
        if self.active {
            // Suspend currently active state.
            if let Some(state) = self.states.last_mut() {
                state.suspend(engine);
            }

            self.states.push(state);
            let state = self.states.last_mut().unwrap();
            state.initialize(engine);
        }
    }
    fn pop(&mut self, engine: &mut Engine<D>) {
        if self.active {
            if let Some(mut state) = self.states.pop() {
                state.cleanup(engine);
            }

            if let Some(state) = self.states.last_mut() {
                state.resume(engine);
            } else {
                self.active = false;
            }
        }
    }
    fn switch(&mut self, engine: &mut Engine<D>, state: Box<State<D>>) {
        if self.active {
            if let Some(mut state) = self.states.pop() {
                state.cleanup(engine);
            }

            self.states.push(state);
            let state = self.states.last_mut().unwrap();
            state.initialize(engine);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::event::Event;

    struct EmptyData;
    impl Data for EmptyData{}

    struct State1(u8, u8);

    struct State2(u8);

    struct State3(u8);

    impl State<EmptyData> for State1 {
        fn initialize(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State1 Initialized");
        }
        fn cleanup(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State1 Cleaned up");
        }
        fn suspend(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State1 Suspended");
        }
        fn resume(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State1 Resumed");
        }
        fn handle(&mut self, _engine: &mut Engine<EmptyData>, event: Event) -> Transition<EmptyData> {
            println!("State1 Handled: {:?}", event);
            Transition::Continue
        }
        fn update(&mut self, _engine: &mut Engine<EmptyData>, delta: f64) -> Transition<EmptyData> {
            println!("State1 Updated: {:?} sec delta", delta);
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
        fn render(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State1 Rendered");
        }
    }

    impl State<EmptyData> for State2 {
        fn initialize(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State2 Initialized");
        }
        fn cleanup(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State2 Cleaned up");
        }
        fn suspend(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State2 Suspended");
        }
        fn resume(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State2 Resumed");
        }
        fn handle(&mut self, _engine: &mut Engine<EmptyData>, event: Event) -> Transition<EmptyData> {
            println!("State2 Handled: {:?}", event);
            Transition::Continue
        }
        fn update(&mut self, _engine: &mut Engine<EmptyData>, delta: f64) -> Transition<EmptyData> {
            println!("State2 Updated {:?} sec", delta);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else {
                Transition::Pop
            }
        }
        fn render(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State2 Rendered");
        }
    }

    impl State<EmptyData> for State3 {
        fn initialize(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State3 Initialized");
        }
        fn cleanup(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State3 Cleaned up");
        }
        fn suspend(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State3 Suspended");
        }
        fn resume(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State3 Resumed");
        }
        fn handle(&mut self, _engine: &mut Engine<EmptyData>, event: Event) -> Transition<EmptyData> {
            println!("State3 Handled: {:?}", event);
            Transition::Continue
        }
        fn update(&mut self, _engine: &mut Engine<EmptyData>, delta: f64) -> Transition<EmptyData> {
            println!("State3 Updated {:?} sec delta", delta);
            if self.0 > 0 {
                self.0 -= 1;
                Transition::Continue
            } else {
                Transition::Pop
            }
        }
        fn render(&mut self, _engine: &mut Engine<EmptyData>) {
            println!("State3 Rendered");
        }
    }

    #[test]
    fn switch_pop() {
        println!("Starting StateManager Test");
        let data = EmptyData;
        let mut engine = Engine::new(data);
        let mut sm = StateManager::new(State1(10, 5));
        sm.start(&mut engine);

        for _ in 0..8 {
            for _ in 0..4 {
                sm.update(&mut engine, 0.1);
            }
            sm.render(&mut engine);
            assert!(sm.active());
        }

        assert!(!sm.active());
    }
}