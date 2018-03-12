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

use super::data::Data;

/// Engine API Layer
pub struct Engine<D: Data> {
    frame_time: f64,
    data: D,
}

impl<D: Data> Engine<D> {
    pub(crate) fn new(data: D) -> Engine<D> {
        let frame_time = 1.0 / 30.0;
        Engine { frame_time, data }
    }
    pub fn data(&mut self) -> &mut D {
        &mut self.data
    }
    pub fn set_fps(&mut self, fps: f64) {
        self.frame_time = 1.0 / fps
    }
    #[inline]
    pub(crate) fn get_frame_time(&self) -> f64 {
        self.frame_time
    }
}
