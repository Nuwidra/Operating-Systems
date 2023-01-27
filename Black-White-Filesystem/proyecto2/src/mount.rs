use std::env;
use std::ffi::OsStr;
use crate::file_system;
use crate::mkfs;
use crate::fsck;
use crate::saved_disk::*;
#[path = "encoder/saved_disk.rs"] mod saved_disk;
pub fn mount() {
    let disk_direction = env::args().nth(2).unwrap();
    let mountpoint = env::args().nth(3).unwrap();
    let mut fs = file_system::bw_disk::new_disk(disk_direction.clone());
    let disk = load_disk(disk_direction.clone());
    fs = file_system::bw_disk::load(disk.unwrap(), fs);
    let options = ["-o", "nonempty"].iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    println!("FS INICIADO");
    fuse::mount(fs, &mountpoint, &options).unwrap();
}