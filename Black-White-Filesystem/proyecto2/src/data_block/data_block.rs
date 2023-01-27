use serde::{Serialize, Deserialize};
// El contenido que llegue a tener el i nodo sera aqui
#[derive(Serialize, Deserialize)]
pub struct Data_block {
    pub reference_i_node : u64,
    pub content: Vec<u8>
}
impl Data_block {
    //Agrega una referencia a si mismo
    pub fn content_add(&mut self,content: u8) {
        self.content.push(content);
    }
    //Elimina una referencia a si mismo
    pub fn content_delete(&mut self,content: u8) {
        self.content.retain(|i| *i != content);
    }
}