use std::env;
use std::mem;
use crate::Disk::*;
use crate::file_system::bw_disk;
use sysinfo::{System, SystemExt};
use crate::saved_disk::*;
#[path = "encoder/saved_disk.rs"] mod saved_disk;
pub fn fsck () {
    let save_path = env::args().nth(2).unwrap();
    if validate_path_FS(save_path.clone()) == false {
        println!("Direccion no valida!"); return; }
    let file_system = load_disk(save_path.clone()).unwrap();
    println!("Cargando Disco Encontrado");

    let mut sys = System::new_all();
    sys.refresh_all();
    println!("TOTAL MEMORY: {} KB", sys.total_memory());
    println!("USED MEMORY: {} KB", sys.used_memory());
    println!("SPACE USED: {} KB", mem::size_of_val(&file_system));
    println!("MEMORY BLOCK SPACE USED: {} KB", mem::size_of_val(&file_system.super_block)*&file_system.super_block.len());
    println!("SUPER BLOCK SPACE USED: {} KB", mem::size_of_val(&file_system.memory_block)*&file_system.memory_block.len());
    println!("SPACE AVAILABLE: {} KB", sys.total_memory()-sys.used_memory());
}