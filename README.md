# TreeVaultDB

Disk-based key-value storage engine written in Rust. It leverages B-tree indexing for rapid data retrieval and write-ahead logging (WAL) to guarantee durability and crash recovery. Designed for performance and reliability

## Storage

Data is stored using B+ trees in binary format to allow for fast reads
