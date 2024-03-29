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

use std::any::Any;
use std::error::Error;

enum ChannelError {
    Full
}

pub trait Publisher<T>
    where
        T: Any + Send + Sync + Sized
{
    /// Offer a value to the queue
    fn offer(&mut self, value: T) -> bool;
    ///
    fn publish(&mut self, value: T);
}

pub trait Subscriber<T>
    where
        T: Any + Send + Sync + Sized
{
    fn poll(&mut self) -> Result<(), Error>;
    fn remove(&mut self) -> Result<T, Error>;
}

pub struct Channel {}

impl Channel {
    pub fn new() -> Channel {}
}