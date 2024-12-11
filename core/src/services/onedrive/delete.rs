// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::backend::OnedriveBackend;
use super::error::parse_error;
use crate::raw::*;
use crate::*;
use http::StatusCode;
use std::sync::Arc;

/// Delete operation
/// Documentation: https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_delete?view=odsp-graph-online
pub struct OnedriveDeleter {
    core: Arc<OnedriveBackend>,
}

impl OnedriveDeleter {
    pub fn new(core: Arc<OnedriveBackend>) -> Self {
        Self { core }
    }
}

impl oio::OneShotDelete for OnedriveDeleter {
    async fn delete_once(&self, path: String, _: OpDelete) -> Result<()> {
        let resp = self.core.onedrive_delete(&path).await?;

        let status = resp.status();

        match status {
            StatusCode::NO_CONTENT | StatusCode::NOT_FOUND => Ok(()),
            _ => Err(parse_error(resp)),
        }
    }
}