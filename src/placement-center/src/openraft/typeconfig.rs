use crate::openraft::raft_node::Node;
use crate::openraft::route::Request;
use crate::openraft::route::Response;
use std::io::Cursor;

pub type SnapshotData = Cursor<Vec<u8>>;

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
        Node = Node,
);
