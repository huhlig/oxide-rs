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
//! Entity Component System
//!
use std::fmt::{Debug, Error, Formatter};

/// Entity ID
type EntityID = usize;

pub struct EntityManager {
    entities: Vec<EntityID>
}

/// Component
pub trait Component: Clone + Copy + Send + Sync + Debug {}

/// System
pub trait System {
    /// Initialize System
    fn initialize();
    ///
    fn cleanup();
    ///
    fn suspend();
    ///
    fn resume();
    ///
    fn update(delta: f64);
}

