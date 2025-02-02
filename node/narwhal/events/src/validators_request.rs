// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValidatorsRequest;

impl EventTrait for ValidatorsRequest {
    /// Returns the event name.
    #[inline]
    fn name(&self) -> Cow<'static, str> {
        "ValidatorsRequest".into()
    }
}

impl ToBytes for ValidatorsRequest {
    fn write_le<W: Write>(&self, _writer: W) -> IoResult<()> {
        Ok(())
    }
}

impl FromBytes for ValidatorsRequest {
    fn read_le<R: Read>(_reader: R) -> IoResult<Self> {
        Ok(Self)
    }
}
