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

use serde::Serialize;

use crate::status_code::StatusCode;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub code: StatusCode,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn new(code: StatusCode, data: T) -> Response<T> {
        Self {
            code,
            msg: format!("{code:?}"),
            data: Some(data),
        }
    }

    pub fn success(data: T) -> Response<T> {
        Self::new(StatusCode::Success, data)
    }

    pub fn error(code: StatusCode, msg: &str) -> Response<T> {
        Self {
            code,
            msg: msg.to_string(),
            data: None,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<T: Serialize> From<StatusCode> for Response<T> {
    fn from(code: StatusCode) -> Self {
        Self {
            code,
            msg: format!("{code:?}"),
            data: None,
        }
    }
}

#[test]
fn test() {
    println!(
        "{}",
        serde_json::to_string_pretty(&std::convert::Into::<Response<()>>::into(
            StatusCode::Success
        ))
        .unwrap()
    );
}
