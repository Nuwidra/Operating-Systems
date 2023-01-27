#[path = "structure/structure_file.rs"] mod strcture_file;
#[path = "i_node/i_node.rs"] mod Node_i;
#[path = "disk/disk.rs"] mod Disk;
#[path = "data_block/data_block.rs"] mod Data_block;
#[path = "file_system/file_system.rs"] mod file_system;
#[path = "encoder/saved_disk.rs"] mod saved_disk;
mod fsck;
mod mkfs;
mod mount;
use std::env;
use std::path::Path;

// ./bwfs fsck fsdir
// ./bwfs mount fsdir mountpoint
// ./bwfs mkfs newfsdir
fn main() {
    if env::args().nth(1).unwrap() == String::from("mount") {
        mount::mount();
    }
    if env::args().nth(1).unwrap() == String::from("fsck") {
        fsck::fsck();
    }
    if env::args().nth(1).unwrap() == String::from("mkfs") {
        mkfs::mkfs();
    }
}