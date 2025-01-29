DROP TABLE IF EXISTS WeaveVMArchiverGoat;
DROP TABLE IF EXISTS WeaveVMArchiverGoatBackfill;

CREATE TABLE IF NOT EXISTS WeaveVMArchiverGoat (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE TABLE IF NOT EXISTS WeaveVMArchiverGoatBackfill (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    NetworkBlockId INT UNIQUE,
    WeaveVMArchiveTxid VARCHAR(66) UNIQUE
);

CREATE INDEX idx_archiver_txid ON WeaveVMArchiverGoat (WeaveVMArchiveTxid);
CREATE INDEX idx_backfill_txid ON WeaveVMArchiverGoatBackfill (WeaveVMArchiveTxid);
CREATE INDEX idx_archiver_block_id ON WeaveVMArchiverGoat (NetworkBlockId);
CREATE INDEX idx_backfill_block_id ON WeaveVMArchiverGoatBackfill (NetworkBlockId);
