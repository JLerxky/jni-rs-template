// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_repr::Serialize_repr;

#[derive(Serialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum StatusCode {
    Error = 0,
    Success = 1,
    UrlError = 2,
    BodyIsNotJson = 3,
    ApiNotExist = 4,
    ParameterError = 5,
}
