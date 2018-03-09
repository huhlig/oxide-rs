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

/// Publisher
pub trait Publisher<T>
    where
        T: Any + Send + Sync
{
    /// Offer a value to the queue, non blocking
    fn offer(&mut self, value: T) -> bool;
    /// Blocks until Successful
    fn add(&mut self, value: T);
}

/// Subscriber
pub trait Subscriber<T>
    where
        T: Any + Send + Sync
{
    /// Request a value, Returning
    fn poll(&mut self) -> Option<T>;
    fn next(&mut self) -> T;
}

/// Watcher
pub trait Watcher<T>
    where
        T: Any + Send + Sync
{
    /// Peek at next Entry if available
    fn peek(&self) -> Option<T>;
    /// Number of current values in Queue
    fn size(&self) -> usize;
}