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
// Gets and parses the options for the tracer from the command line interface.
//
// Inputs: String written in the command line interface.
//
// Ouput: Tuple with:
//      1. Vector of strings with the prog arguments.
//      2. Tracer execution mode.
// ======================================================================================
pub fn parse_options(args: Vec<String>) -> Vec<String> {
     
    let host_option = "h";

    // ======================================================================================
    // Create a new option
    // ======================================================================================
    let mut opts = Options::new();
    
    // ======================================================================================
    // Tracer possible options
    // ======================================================================================
    opts.optflag(host_option, "", "Host to connect to on the server");

    // ======================================================================================
    // Verifies mode for tracer
    // ======================================================================================
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    return matches.free;    

}