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

extern crate getopts;
use getopts::Options;

// ======================================================================================
// The parse_options will parse the options of the web server later they will be explained
// ======================================================================================
pub fn parse_options(args: Vec<String>) -> Vec<String> {

    // ======================================================================================
    // Declaration of variables
    // ======================================================================================
    let thread_number_opt = "n";
    let resource_directory_opt = "w";
    let port_opt = "p";
    let mut opts = Options::new();
    
    // ======================================================================================
    // Tracer possible options  
    // ====================================================================================== 
    opts.optflag(thread_number_opt, "", "Number of threads for the web server");
    opts.optflag(resource_directory_opt, "", "Directory to serve resources");
    opts.optflag(port_opt, "", "Port to listen on");
    
    // ======================================================================================
    // Verifies mode for tracer
    // ======================================================================================
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    return matches.free;

    

    // ======================================================================================
    // Args has the rest of the prog information (program to track and arguments). 
    // ======================================================================================
}