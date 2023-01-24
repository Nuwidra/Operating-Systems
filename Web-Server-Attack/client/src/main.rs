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

#[path = "get_options/get_optios.rs"]
mod get_options;

#[path = "client/client.rs"]
mod client;

use std::env;

#[tokio::main]
async fn main() {
    
    // ======================================================================================
    // Create vectors saved in a collection
    // ======================================================================================
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    // ======================================================================================
    // Options are parsed
    // ======================================================================================
    let options = get_options::parse_options(args);

    let host = options.get(0).unwrap();
    let method = options.get(1).unwrap().to_string();
    let resource = options.get(2).unwrap();

    // ======================================================================================
    // In the case that is GET
    // ======================================================================================
    if method == "get" {
        client::get(host, resource).await;
    }

    // ======================================================================================
    // In the case that is HEAD
    // ======================================================================================
    if method == "head" { 
        client::head(host, resource).await;
    }

    // ======================================================================================
    // In the case that is DELETE
    // ======================================================================================
    if method == "delete" { 
        client::delete(host, resource).await;
    }

    // ======================================================================================
    // In the case that is PUT
    // ======================================================================================
    if method == "put" {
        client::put(host, resource).await;
    }

    // ======================================================================================
    // In the case that is POST
    // ======================================================================================
    if method == "post" {
        client::post(host, resource).await;
    }


}
