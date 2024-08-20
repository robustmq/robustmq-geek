// Copyright 2023 RobustMQ Team
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    fs,
    path::{self, Path},
};

use crate::errors::RobustMQError;

pub fn create_fold(fold: &String) -> Result<(), RobustMQError> {
    if !Path::new(fold).exists() {
        fs::create_dir_all(fold)?
    }
    return Ok(());
}

pub fn file_exists(path: &String) -> bool {
    return Path::new(path).exists();
}

pub fn read_file(path: &String) -> Result<String, RobustMQError> {
    if !path::Path::new(path).exists() {
        return Err(RobustMQError::CommmonError(format!(
            "File {} does not exist",
            path
        )));
    }

    return Ok(fs::read_to_string(&path)?);
}
