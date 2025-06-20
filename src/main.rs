use rust_rocksdb::{
    MultiThreaded, OptimisticTransactionDB, OptimisticTransactionOptions, SnapshotWithThreadMode,
    TransactionDBOptions, TransactionOptions, WriteOptions,
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
    std::fs::remove_dir_all("/tmp/ckb_rocksdb").unwrap();
    let db: &'static Arc<OptimisticTransactionDB<MultiThreaded>> = get_txn_db();
    db.put("1", "2").unwrap();

    let txn1 = db.transaction();
    txn1.put("2", "3").unwrap();
    txn1.commit().unwrap();

    let snapshot: SnapshotWithThreadMode<'static, OptimisticTransactionDB<MultiThreaded>> =
        get_snapshot();

    let txs2 = db.transaction();
    txs2.put("3", "4").unwrap();
    txs2.commit().unwrap();

    assert_eq!(snapshot.get("1").unwrap(), Some("2".as_bytes().to_vec()));
    assert_eq!(
        db.transaction().get("2").unwrap(),
        Some("3".as_bytes().to_vec())
    );
    assert_eq!(snapshot.get("3").unwrap(), None);
}
