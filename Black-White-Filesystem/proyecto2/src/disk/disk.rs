use crate::Node_i::*;
use crate::Data_block::*;
use fuse::{FileType, FileAttr};
use serde::{Serialize, Deserialize};
// El disco sera un vector con todos los i nodos
#[derive(Serialize, Deserialize)]
pub struct Disk {
    pub super_block: Vec<Node_i>,
    pub memory_block: Vec<Data_block>,
    pub path: String,
}

impl Disk {
    // Se crea un nuevo disco
    pub fn new_disk(path:String) -> Disk {
        // Son los bloques de los datos
        let mut memory_block = Vec::new();
        // La información que pueda tener los i nodos estaran en esta variable
        let mut blocks = Vec::new(); 
        let ts = time::now().to_timespec();
        let structure_attributes = FileAttr { ino: 1, size: 0, blocks: 0, atime: ts, mtime: ts, ctime: ts, crtime: ts, kind: FileType::Directory, perm: 0o755, nlink: 0, uid: 0, gid: 0, rdev: 0, flags: 0,};
        let name = "None";
        let first_node = Node_i {
            name : name.to_string(),
            attributes : structure_attributes,
            references : Vec::new()
        };
        blocks.push(first_node);
        let new_disk = Disk {super_block: blocks, memory_block, path};
        return new_disk;
    }
    // Avanza con el siguiente i nodo
    pub fn next_node_i(&mut self) -> u64 {
        let superblock_length = self.super_block.len() + 1;
        return (superblock_length) as u64;
    }
    // Se escribe la informacion del i nodo
    pub fn write_node_i(&mut self, Node_i:Node_i) {
        self.super_block.push(Node_i);
    }
    // Se elimina la informacion del i nodo
    pub fn remove_node_i(&mut self, Node_i:u64) {
        self.super_block.retain(|i| i.attributes.ino != Node_i);
    }
    // ----------------------------------------------------------------------------------------
    // Elimina una referencia de un i nodo 
    pub fn delete_reference(&mut self, i_node: u64, reference_value: usize) {
        let superblock_length = self.super_block.len();
        for node in 0..superblock_length {
            if self.super_block[node].attributes.ino == i_node {
                let reference_copy = reference_value.clone();
                let node_copy = node.clone();
                self.super_block[node_copy].delete_reference(reference_copy);
            }
        }
    }
    // Se añade una referencia de un i nodo
    pub fn add_reference(&mut self, i_node: u64, reference_value: usize) {
        let superblock_length = self.super_block.len();
        for node in 0..superblock_length {
            let super_block_attributes = self.super_block[node].attributes.ino;
            if super_block_attributes == i_node {
                let reference_copy = reference_value.clone();
                let node_copy = node.clone();
                self.super_block[node_copy].add_reference(reference_copy);
            }
        }
    }
    // ----------------------------------------------------------------------------------------
    // Se obtiene el i nodo apartir del super bloque correspondiente
    pub fn obtain_node_i (&self, i_node: u64) -> Option<&Node_i> {
        let superblock_length = self.super_block.len();
        for node in 0..superblock_length {
            if self.super_block[node].attributes.ino == i_node {
                let node_copy = node.clone();
                return Some(&self
                    .super_block[node_copy]);
            } else {
                return None;
            }
        }
        return None;
    }
    // Se obtiene el i nodo mutable apartir del super bloque correspondiente
    pub fn obtain_mutable_node_i (&mut self, i_node: u64) -> Option<&mut Node_i> {
        let superblock_length = self.super_block.len();
        for node in 0..superblock_length {
            if self.super_block[node].attributes.ino == i_node {
                let mutable_node = node.clone();
                return Some(&mut self
                    .super_block[mutable_node]);
            } else {
                return None;
            }
        }
        return None;
    }
    // Se adiciona los datos del i nodo por medio del identificador
    pub fn add_data_to_node(&mut self, i_node:u64, data:u8) {
        let memory_block_length = self.memory_block.len();
        for node in 0..memory_block_length {
            if self.memory_block[node].reference_i_node == i_node {
                let data_copy = data.clone();
                let node_copy = node.clone();
                self.memory_block[node_copy]
                .content_add(data_copy) ;
            }
        }
    }
    // Se escribe los datos que tenga los i nodos en el data block
    pub fn write_data_to_node(&mut self, i_node_reference: u64, content: Vec<u8>) {
        let node_reference_copy = i_node_reference.clone();
        for node in 0..content.len(){
            self.add_data_to_node(node_reference_copy, 
                content[node].clone());

        }
    }
    // Se obtiene la cantidad de bytes del arreglo para el data block
    pub fn get_bytes_content(&self, i_node: u64) -> Option<&[u8]> {
        let data_block_length = self.memory_block.len();
        for i in 0..data_block_length {
            let reference_i_node = self.memory_block[i].reference_i_node;
            if reference_i_node == i_node {
                let content_bytes = &self.memory_block[i].content[..];
                let bytes = content_bytes;
                return Some(bytes);
            } else {
                return None;
            }
        } return None;
    }

    // Se encuentra el i nodo respectivo a partir de su identificador
    pub fn find_node_i_in_references_by_name(&self, parent: u64, name: &str) -> Option<&Node_i> {
        let super_block_length = self.super_block.len();
        for block in 0..super_block_length {
            let each_block = self.super_block[block].attributes.ino;
            if each_block == parent {
                let parent =  &self.super_block[block.clone()];
                let parent_reference_length = parent.references.len();
                for reference in 0..parent_reference_length {
                    for new_block in 0..super_block_length {
                        let attribute_node_i = self.super_block[new_block].attributes.ino;
                        let parent_reference = parent.references[reference.clone()].try_into().unwrap();
                        if attribute_node_i == parent_reference {
                            let super_block_copy = &self.super_block[new_block.clone()];
                            let child =  super_block_copy;
                            // Cuando el nombre es igual a que se esta buscando
                            if child.name == name {
                                return Some(child);
                            }}}}}} return None;
    }

    // ----------------------------------------------------------------------------------------
}