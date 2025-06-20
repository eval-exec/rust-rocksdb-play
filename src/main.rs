use rust_rocksdb::{
    DBWithThreadMode, MultiThreaded, OptimisticTransactionDB, Snapshot, SnapshotWithThreadMode,
};
use std::sync::{Arc, OnceLock};

static DB: OnceLock<Arc<OptimisticTransactionDB<MultiThreaded>>> = OnceLock::new();

fn get_txn_db() -> &'static Arc<OptimisticTransactionDB<MultiThreaded>> {
    DB.get_or_init(|| {
        let dir = std::path::Path::new("/tmp/ckb_rocksdb");
        let mut opts = rust_rocksdb::Options::default();
        opts.create_if_missing(true);
        Arc::new(rust_rocksdb::OptimisticTransactionDB::open(&opts, dir).unwrap())
    })
}

fn get_snapshot() -> SnapshotWithThreadMode<'static, OptimisticTransactionDB<MultiThreaded>> {
    let snapshot: SnapshotWithThreadMode<'static, OptimisticTransactionDB<MultiThreaded>> =
        get_txn_db().snapshot();
    snapshot
}

fn main() {
    let db: &'static Arc<OptimisticTransactionDB<MultiThreaded>> = get_txn_db();
    db.put("hello", "world").unwrap();

    {
        let txn = db.transaction();
        txn.put("1", "2").unwrap();
        txn.commit().unwrap();
    }

    let snapshot: SnapshotWithThreadMode<'static, OptimisticTransactionDB<MultiThreaded>> =
        get_snapshot();
    let result = snapshot.get("hello").unwrap();
    assert_eq!(result, Some("world".as_bytes().to_vec()));
    dbg!(String::from_utf8(result.unwrap()).unwrap());
}
