use std::path::Path;
use std::sync::Arc;

use super::typeconfig::TypeConfig;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use log_store::LogStore;
use openraft::{SnapshotMeta, StorageError};
use rocksdb::ColumnFamilyDescriptor;
use rocksdb::Options;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use state_machine_store::StateMachineStore;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredSnapshot {
    pub meta: SnapshotMeta<TypeConfig>,

    /// The data of the state machine at the time of this snapshot.
    pub data: Vec<u8>,
}

type StorageResult<T> = Result<T, StorageError<TypeConfig>>;

pub mod log_store;
pub mod state_machine_store;

/// converts an id to a byte vector for storing in the database.
/// Note that we're using big endian encoding to ensure correct sorting of keys
fn id_to_bin(id: u64) -> Vec<u8> {
    let mut buf = Vec::with_capacity(8);
    buf.write_u64::<BigEndian>(id).unwrap();
    buf
}

fn bin_to_id(buf: &[u8]) -> u64 {
    (&buf[0..8]).read_u64::<BigEndian>().unwrap()
}

pub(crate) async fn new_storage<P: AsRef<Path>>(db_path: P) -> (LogStore, StateMachineStore) {
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);

    let store = ColumnFamilyDescriptor::new("store", Options::default());
    let logs = ColumnFamilyDescriptor::new("logs", Options::default());

    let db = DB::open_cf_descriptors(&db_opts, db_path, vec![store, logs]).unwrap();
    let db = Arc::new(db);

    let log_store = LogStore { db: db.clone() };
    let sm_store = StateMachineStore::new(db).await.unwrap();

    (log_store, sm_store)
}
