/* ======================================================================================
 
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
			
Proyecto 1: Multidisplay Animator

// ====================================================================================== */

// ======================================================================================
// Ejecución en Terminal para el funcionamiento del proyecto:
//                       [1] "./animar -c config.txt"
// ======================================================================================

// ======================================================================================
// Bibliotecas necesarias para la animación, movimiento y el pintado de la misma
// ======================================================================================


extern crate core;
// ======================================================================================
// Se importa la biblioteca my_pthread
// ======================================================================================
mod my_pthread;
mod animator;
use animator::animator;
use std::env;
fn main () {
    let mut args: Vec<_> = env::args().collect();
    println!("{:?}", args[1]);
    if args[1] != "-c" {
        panic!("Argumentos incorrectos");
    }
    animator(args[2].clone());
}