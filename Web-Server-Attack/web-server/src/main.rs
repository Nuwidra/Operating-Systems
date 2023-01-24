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
// ====================================================================================== :D */

#[path = "get_ops/get_options.rs"]
mod get_options;

#[path = "http_server/http_handler.rs"]
mod http;


use threadpool::ThreadPool;
use std::env;
// ======================================================================================
// This main function will execute everything that the web server has to do together with the client.
// ======================================================================================
fn main() {
    //let path = "/home/beto/Documents/Repositories/Sistemas Operativos/tarea3/ic-6600-t3-webServerAttack/web-server/resources";
    //let threads = 2;

    // ======================================================================================
    // The arguments saved in a collection are created
    // ======================================================================================
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    // ======================================================================================
    // Options are parsed
    // ======================================================================================
    let program = get_options::parse_options(args);

    let threads = program.get(0).unwrap().parse::<usize>().unwrap();
    let port = program.get(2).unwrap().to_string();

    //println!("{:?} {:?}", threads, port);

    // ======================================================================================
    // A ThreadPool is created to manage the threads of execution
    // ======================================================================================
    let thread_pool = ThreadPool::new(threads);

    // ======================================================================================
    // The connection to the web server is created
    // ======================================================================================
    let listener = http::create_connection(port);

    // ======================================================================================
    // The connection is handled as far as requests are concerned
    // ======================================================================================
    http::handle_connection(listener, thread_pool);
}
