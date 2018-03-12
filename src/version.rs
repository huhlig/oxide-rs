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

use std::fmt::{Display, Debug, Formatter, Result};

pub struct Version {
    major: &'static str,
    minor: &'static str,
    patch: &'static str,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "v{}.{}.{})", self.major, self.minor, self.patch)
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "v{}.{}.{})", self.major, self.minor, self.patch)
    }
}

pub const VERSION: Version = Version {
    major: env!("CARGO_PKG_VERSION_MAJOR"),
    minor: env!("CARGO_PKG_VERSION_MINOR"),
    patch: env!("CARGO_PKG_VERSION_PATCH"),
};