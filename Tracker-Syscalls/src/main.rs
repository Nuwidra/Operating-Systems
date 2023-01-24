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
Alumno: 
        Jonathan Quesada Salas
			
Tarea Corta 1: Rastreador de System Calls
*/
// ====================================================================================== */

// ======================================================================================
// Ejecución en Terminal para el funcionamiento de la tarea:
//                       [1] "cargo build"
//                       [2] "./target/debug/sys_call_tracer -V ls"
// ======================================================================================

// ======================================================================================
// Se importar los todos lo modulos para la realización del main
// ======================================================================================
mod argument;
mod argument_parser;
mod arguments_summary;
mod runner;
mod tracer;
// ======================================================================================
// Se llaman las funciones e implimentaciones principales de cada modulo
// ======================================================================================
use argument_parser::parser::Parser as Parser;
use tracer::Tracer as Tracer;
use arguments_summary::ArgumentsSummary as ArgumentsSummary;
use runner::t_runner::TRunner as TRunner;
use argument_parser::t_parser::TParser as TParser;

use std::env;

// 
fn main() {
    // ======================================================================================
    // Se crea una colección para poder almacenar los argumentos del mensaje ingresado
    // ======================================================================================
    let args: Vec<String> = env::args().collect();
    // ======================================================================================
    // Se crea un parser para poder almacenar los parametros
    // ======================================================================================
    let parser = Parser::new();
    // ======================================================================================
    // Se analizan los argumentos del parseo y se guardan en una variable
    // ======================================================================================
    let parsed = parser.parse(args);
    // ======================================================================================
    // Se vefica los parámetros en dado caso que esten mal ingresados retorna error
    // ======================================================================================
    if let Err(e) = parsed {
        println!("Error: {}", e);
    }
    else {
        // ======================================================================================
        // En dado caso que los argumentos esten bien ingresados se procede a llamar a 
        // ArgumentsSummary el cual se encarga de recibir una lista de argumentos y resumirlos 
        // en un objeto
        // ======================================================================================
        let tracer_summary = ArgumentsSummary::summarize(parsed.unwrap());
        // ======================================================================================
        // Seguidamente se procede a llamar y crear un objeto tracer el cual contendrá 
        // los argumentos de ArgumentsSummary para luego en el modulo tracer saber como poder
        // imprimir los syscalls
        // ======================================================================================
        let mut tracer = Tracer::new(tracer_summary);
        // ======================================================================================
        // Simplemente se ejecuta el tracer con ayuda con el modulo runner
        // ======================================================================================
        tracer.execute();
    }
}


