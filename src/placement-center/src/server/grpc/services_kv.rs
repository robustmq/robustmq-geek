// Copyright 2023 RobustMQ Team
//
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

use dashmap::DashMap;
use protocol::kv::{
    kv_service_server::KvService, CommonReply, DeleteRequest, ExistsReply, ExistsRequest, GetReply,
    GetRequest, SetRequest,
};
use tonic::{Request, Response, Status};

pub struct GrpcKvServices {
    data: DashMap<String, String>,
}

impl GrpcKvServices {
    pub fn new() -> Self {
        return GrpcKvServices {
            data: DashMap::with_capacity(8),
        };
    }
}

#[tonic::async_trait]
impl KvService for GrpcKvServices {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<CommonReply>, Status> {
        let req = request.into_inner();
        self.data.insert(req.key, req.value);
        return Ok(Response::new(CommonReply::default()));
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        let req = request.into_inner();
        if let Some(data) = self.data.get(&req.key) {
            return Ok(Response::new(GetReply {
                value: data.value().clone(),
            }));
        }
        return Ok(Response::new(GetReply::default()));
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<CommonReply>, Status> {
        let req = request.into_inner();
        self.data.remove(&req.key);
        return Ok(Response::new(CommonReply::default()));
    }

    async fn exists(
        &self,
        request: Request<ExistsRequest>,
    ) -> Result<Response<ExistsReply>, Status> {
        let req = request.into_inner();
        return Ok(Response::new(ExistsReply {
            flag: self.data.contains_key(&req.key),
        }));
    }
}


