use fuse::FileAttr;
use serde::{Serialize, Deserialize};
use crate::strcture_file::FileAttrDef;
// Estructura de los i nodos
#[derive(Serialize, Deserialize)]
pub struct Node_i {
    pub name: String,
    #[serde(with = "FileAttrDef")]
    pub attributes: FileAttr,
    pub references: Vec<usize>
}

impl Node_i {
    // Agrega una determinada referencia de un i nodo
    pub fn add_reference(&mut self,ref_value: usize) {
        self.references.push(ref_value);
    }
    // Elimina una determinada referencia de un i nodo
    pub fn delete_reference(&mut self,ref_value: usize) {
        self.references.retain(|i| *i != ref_value);
    }
    // Cambia una determinada referencia de un i nodo
    pub fn change_name(&mut self,value: String) {
        self.name = value;
    }
}
