use std::env;
use std::path::Path;
use std::fs;
use crate::file_system::{bw_disk, self};
pub fn mkfs() {
    let save_path = env::args().nth(2).unwrap();
    if Path::new(save_path.as_str()).exists() {fs::remove_dir_all(save_path.clone()).unwrap();}
    fs::create_dir_all(save_path.clone()).unwrap();
    file_system::bw_disk::new_disk(save_path.clone());
    println!("FS Creado");
}