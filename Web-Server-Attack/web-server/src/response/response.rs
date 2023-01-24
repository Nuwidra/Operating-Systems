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

// ======================================================================================
// The enum will have the structure of ResponseStatus which will have two options OK and NotFound
// ======================================================================================
pub enum ResponseStatus {
    OK,
    //NotImplemented,
    NotFound,
    //InternalServerError,
}

impl ResponseStatus {
    
    // ======================================================================================
    // The state that the web server may have is obtained
    // ======================================================================================
    fn get_status(&self) -> (i32, String) {
        match &self {
            ResponseStatus::OK => return (200, "OK".to_string()),
            //ResponseStatus::NotImplemented => return (501, "NOT IMPLEMENTED".to_string()),
            ResponseStatus::NotFound => return (404, "NOT FOUND".to_string()),
            //ResponseStatus::InternalServerError => return (500, "INTERNAL SERVER ERROR".to_string()),
        }
    }
    
    // ======================================================================================
    // The code is obtained
    // ======================================================================================
    pub fn get_code(&self) -> i32 {
        let status = &self.get_status();
        return status.0;
    }
    
    // ======================================================================================
    // The message from the web server that you may have depending on the request entered is obtained
    // ======================================================================================
    pub fn get_message(&self) -> String {
        let status = self.get_status();
        return status.1;   
    }
    
}

// ======================================================================================
// The response is constructed by shaping it in terms of the requested format
// ======================================================================================
pub fn generate_response(status_line: String, content_length: usize, content: String) -> String {
    
    // ======================================================================================
    // Response format as such
    // ======================================================================================
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", 
        status_line, 
        content_length, 
        content);
    
    // ======================================================================================
    // The response is returned
    // ======================================================================================
    return response;

}


