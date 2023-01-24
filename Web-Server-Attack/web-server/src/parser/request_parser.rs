/* ======================================================================================
/* 
   ____    ____           ___    ____   ___   ___
  / ___/  / __ \         |__ \  / __ \ |__ \ |__ \ 
  \__ \  / / / /  _____  __/ / / / / / __/ / __/ /
 ___/ / / /_/ /  /____/ / __/ / /_/ / / __/ / __/
/____/  \____/         /____/ \____/ /____//____/
Instituto Tecnológico de Costa Rica
Carrera: 
        Bachillerato en Ingeniería en Computación
Curso: 
        Principios de Sistemas Operativos
Profesor: 
        Kevin Moraga García
Alumnos: 
        Alberto Zumbado Abarca
        Jonathan Quesada Salas
			
Tarea Corta 3: Web Server Attack
*/
// ====================================================================================== */

#[path = "../request/request.rs"]
pub mod request;

use request::Request;
use std::net::TcpStream;
use std::io::Read;

// ======================================================================================
// This function will take care of building the request
// ======================================================================================
pub fn build_request(mut connection_stream: &TcpStream) -> Request {
    
    // ======================================================================================
    // The buffer reader is declared
    // ======================================================================================
    let mut buffer_reader = [0; 1024];

    // ======================================================================================
    // the connection is read
    // ======================================================================================
    connection_stream.read(&mut buffer_reader).unwrap();
    
    let request_string = String::from_utf8_lossy(&buffer_reader[..]);
    
    // ======================================================================================
    // split_whitespace() method of a string in Rust splits a string into whitespace substrings
    // ======================================================================================
    let mut request_line = request_string.split_whitespace();
    
    // ======================================================================================
    // The method to be used in the request is stored
    // ======================================================================================
    let method = request_line.next().unwrap().to_string();


    // ======================================================================================
    // Read the request line by line...
    // ======================================================================================
    let mut resource = request_line.next().unwrap().to_string();
    
    // ======================================================================================
    // In case there is a resource
    // ======================================================================================
    if resource == "/" {
        resource = "/index.html".to_string();
    }
    
    // ======================================================================================
    // The request is declared and returned
    // ======================================================================================
    let request = Request { method: (method), resource: (resource), full_body: (request_string.to_string()) };
    
    return request;
    
    
}
