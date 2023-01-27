use crate::Data_block::*;
use crate::Disk::*;
use crate::Node_i::*;
use crate::saved_disk::encode;
use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyStatfs,ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use libc::{ENOSYS, ENOENT, EIO};
use std::ffi::OsStr;
use std::mem;
use crate::saved_disk::*;
#[path = "../encoder/saved_disk.rs"] mod save_path;
pub struct bw_disk {
    bw_disk: Disk
}
impl bw_disk {
    pub fn new_disk(path:String) -> Self {
        bw_disk { bw_disk: Disk::new_disk(path) }
    }
    pub fn load(disk:Disk,mut fs:bw_disk) -> bw_disk{
        fs.bw_disk = disk;
        return fs
    }
    pub fn get_disk(&self) -> &Disk {
        return &self.bw_disk;
    }
    pub fn set_disk(&mut self,new_disk:Disk) {
        self.bw_disk = new_disk;
    }
    pub fn file_system_save(&self){
        let encode_fs = encode(&self.bw_disk);
        write_pixels(1000,1000,encode_fs,&self.bw_disk.path, 0, 0)
    }
}
impl Drop for bw_disk {
    fn drop(&mut self) {
        &self.file_system_save();
    }
}
impl Filesystem for bw_disk {
    
    fn getattr(&mut self,_req: &Request, ino: u64, reply: ReplyAttr) {
        let inode = self.bw_disk.obtain_node_i(ino);
        match inode {
            Some(inode) => {
                reply.attr(&time::now().to_timespec(), &inode.attributes);
                print!("Getattr Successful")
            },
            None => reply.error(ENOENT)
        }
    }
    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, flags: u32, reply: ReplyCreate) {
        let available_i_node = self.bw_disk.next_node_i();
        let data_block = Data_block { reference_i_node : self.bw_disk.next_node_i(), content : Vec::new()};
        let attr = FileAttr { ino: self.bw_disk.next_node_i(), size: 0, blocks: 1, atime: time::now().to_timespec(), mtime: time::now().to_timespec(), ctime: time::now().to_timespec(), crtime: time::now().to_timespec(), kind: FileType::RegularFile, perm: 0o755, nlink: 0, uid: 0, gid: 0, rdev: 0, flags,};
        let name = name.to_str().unwrap();
        let mut inode = Node_i { name: (name.to_string()), attributes: (attr), references: (Vec::new()) };
        inode.references.push(data_block.reference_i_node as usize);
        self.bw_disk.write_node_i(inode);
        self.bw_disk.add_reference(parent, available_i_node as usize);
        self.bw_disk.memory_block.push(data_block);
        reply.created(&time::now().to_timespec(), &attr, 1, self.bw_disk.next_node_i(), flags)
    }

    fn open(&mut self, _req: &Request, _ino: u64, _flags: u32, reply: ReplyOpen) {
        let _data_block = self.bw_disk.get_bytes_content(_ino);
        match self.bw_disk.get_bytes_content(_ino) {
            Some(_data_block) => {reply.opened(1, 0);},
            None => reply.error(ENOENT)
        }
    }
    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, _offset: i64, _size: u32, reply: ReplyData) {
        match self.bw_disk.get_bytes_content(ino) {
            Some(data_block) => {reply.data(data_block); },
            None => {reply.error(EIO);}
        }
    }
    fn write(&mut self, _req: &Request, ino: u64, _fh: u64, _offset: i64, data: &[u8], _flags: u32, reply: ReplyWrite) {

        let content: Vec<u8> = data.to_vec();
        match self.bw_disk.obtain_mutable_node_i(ino) {
            Some(inode) => {
                let length_data = data.len();
                inode.attributes.size = length_data as u64;
                self.bw_disk.write_data_to_node(ino.clone(), content);
                reply.written(length_data as u32);},
            None => {reply.error(ENOENT);}
        }
    }
    fn rename(&mut self, _req:&Request, parent:u64, name:&OsStr, _newparent: u64, newname:&OsStr, reply:ReplyEmpty) {
        let name = name.to_str().unwrap();
        match self.bw_disk.find_node_i_in_references_by_name(parent, name) {
            Some(inode) => {
                match self.bw_disk.obtain_mutable_node_i(inode.attributes.ino) {
                    Some(child) => { child.name = newname.to_str().unwrap().to_string(); reply.ok()},
                    None =>{reply.error(ENOENT);}}
            },
            None =>{reply.error(ENOENT);}
        }
    }
    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        let ino = self.bw_disk.next_node_i();
        let attr = FileAttr { ino: self.bw_disk.next_node_i() as u64, size: 0, blocks: 1, atime: time::now().to_timespec(), mtime: time::now().to_timespec(), ctime: time::now().to_timespec(), crtime: time::now().to_timespec(), kind: FileType::Directory, perm: 0o755, nlink: 0, uid: 0, gid: 0, rdev: 0, flags: 0,};
        let name = name.to_str().unwrap().to_string();
        let inode = Node_i { name, attributes: attr, references: Vec::new() };
        self.bw_disk.write_node_i(inode);
        self.bw_disk.add_reference(parent,ino.clone() as usize);
        reply.entry(&time::now().to_timespec(), &attr, 0);
    }
    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino == 1 {
            if offset == 0 {
                let file_directory = FileType::Directory;
                reply.add(1, 0, file_directory, ".");
                reply.add(1, 1, file_directory, "..");
            }
        }
        if mem::size_of_val(&self.bw_disk.obtain_node_i(ino)) == offset as usize { reply.ok(); return; }

        match self.bw_disk.obtain_node_i(ino) {
            Some(inode) => {
                for ino in &inode.references {
                    if let Some(inode_data) = self.bw_disk.obtain_node_i(*ino as u64) {
                        if inode_data.attributes.ino == 1 { continue;}
                        reply.add(inode_data.attributes.ino, mem::size_of_val(&self.bw_disk.obtain_node_i(*ino as u64)) as i64, inode_data.attributes.kind, &inode_data.name);}
                } reply.ok()},
            None => { println!("ERROR") }
        }
    }
    fn opendir(&mut self, _req: &Request, _ino: u64, _flags: u32, reply: ReplyOpen) {
        match self.bw_disk.obtain_node_i(_ino) {
            Some(dir) => {
                let attribute = dir.attributes.ino;
                reply.opened(attribute, 1 as u32); },
            None => {println!("Error");}
        }
    }
    fn rmdir(&mut self,_req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = name.to_str().unwrap();
        match self.bw_disk.find_node_i_in_references_by_name(parent, name) {
            Some(inode) => {
                let ino = inode.attributes.ino;
                self.bw_disk.delete_reference(parent.clone(), (inode.attributes.ino) as usize);
                self.bw_disk.remove_node_i(ino);
                reply.ok(); },
            None => reply.error(EIO)
        }
    }
    fn statfs(&mut self, _req: &Request, _ino: u64, reply: ReplyStatfs) {
        let blocks:u64 =  (self.bw_disk.super_block.len() + self.bw_disk.memory_block.len()) as u64;
        let bfree:u64 = blocks - self.bw_disk.memory_block.len() as u64;
        let bavail:u64 = bfree;
        let files:u64 = self.bw_disk.memory_block.len().try_into().unwrap();
        let ffree:u64 = 1024 as u64;
        let bsize:u32 = (mem::size_of::<Vec<Node_i>>() as u32 +mem::size_of::<Node_i>() as u32)*1024;
        let namelen:u32 = 77;
        let frsize:u32 = 1;
        reply.statfs(blocks.clone(), bfree.clone(), bavail, files, ffree, bsize, namelen, frsize);
    }
    fn fsync(&mut self, _req: &Request, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) { 
        reply.error(ENOSYS);
    }
    fn access(&mut self, _req: &Request, _ino: u64, _mask: u32, reply: ReplyEmpty) {
        reply.ok();
    }
    /* 
    fn unlink(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = name.to_str().unwrap();
        let inode = self.bw_disk.find_node_i_in_references_by_name(parent, name);

        match inode {
            Some(inode) => {
                if inode.attributes.kind == FileType::Directory {
                    reply.error(ENOSYS);
                } else {
                    let ino = inode.attributes.ino;
                    let memory_block_index = (ino as usize) - 1;
                    self.bw_disk.clear_inode(ino);
                    self.bw_disk.clear_memory_block(memory_block_index);
                    self.bw_disk.clear_reference_in_inode(parent, ino as usize);
                    reply.ok()
                }
            },
            None => reply.error(EIO)
        }
    }*/
    fn flush(&mut self, _req: &Request, _ino: u64, _fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        reply.error(ENOSYS);
    }
}