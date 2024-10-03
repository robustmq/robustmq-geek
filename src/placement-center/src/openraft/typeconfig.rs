use crate::openraft::raft_node::Node;
use crate::openraft::route::AppRequestData;
use crate::openraft::route::AppResponseData;
use std::io::Cursor;

pub type SnapshotData = Cursor<Vec<u8>>;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = AppRequestData,
        R = AppResponseData,
        Node = Node,
);
