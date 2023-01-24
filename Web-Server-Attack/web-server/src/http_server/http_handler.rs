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

#[path = "../parser/request_parser.rs"]
mod request_parser;

#[path = "../response/response.rs"]
mod response;

#[path = "../get_ops/get_options.rs"]
mod get_options;


use std::fs::remove_file;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use response::ResponseStatus;
use threadpool::ThreadPool;
use std::env;
use std::thread; 
use std::time::Duration;


static mut THREADS: i32 = 0;


// struct Threads {
//     number: i32,

// }

//const PORT: &str = "127.0.0.1:7878";

// ======================================================================================
// A new connection is created in the browser
// ======================================================================================
pub fn create_connection(port: String) -> TcpListener {

    let connetion_listener = TcpListener::bind(port).unwrap();
    return connetion_listener;

}

// ======================================================================================
// handle_connection will have input: socket_stream and threadpool for handling the connection on the web server
// ======================================================================================
pub fn handle_connection(socket_stream: TcpListener, threadpool: ThreadPool) {
    
    unsafe {
        THREADS = threadpool.max_count().to_string().parse().unwrap();
        println!("Number of threads {:?}", THREADS);
    }
    // ======================================================================================
    // This cycle will determine the connection of the stream in the socket
    // ======================================================================================
    for connection_stream in socket_stream.incoming() {
        let stream = connection_stream.unwrap();

        unsafe {
            THREADS -= 1;  
            print!("{:?}\n", THREADS);  

            // ======================================================================================
            // In case you can't connect
            // ======================================================================================
            if THREADS < 0 {

                println!("\n Cannot proccess connection, number of threads are unavailable\n ");
                continue;
            } ;

        }

        
        threadpool.execute(|| {

            // ======================================================================================
            // The vectors are parsed to save them in a collection
            // ======================================================================================
            let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

            // ======================================================================================
            // Options are parsed
            // ======================================================================================
            let options = get_options::parse_options(args);
        
            // ======================================================================================
            // It's called handle_stream
            // ======================================================================================
            handle_stream(stream, options.get(1).unwrap().to_string());

        });
    }
}

// ======================================================================================
// handle_stream will have only 2 entries which will be the stream and the resources_path
// ======================================================================================
fn handle_stream(mut stream: TcpStream, resources_path: String) {

    // ======================================================================================
    // build the parsing of the options for the request
    // ======================================================================================
    let request = request_parser::build_request(&stream);
    println!("{}", request.full_body);

    // ======================================================================================
    // The response is declared as a string
    // ======================================================================================
    let response: String;

    // ======================================================================================
    // In the case that the request is GET
    // ======================================================================================
    if request.method == "GET" {
        response = get_method(request, resources_path);
    }

    // ======================================================================================
    // In the case that the request is HEAD
    // ======================================================================================
    else if request.method == "HEAD" {
        response = head_method(request, resources_path);
    }

    // ======================================================================================
    // In the case that the request is DELETE
    // ======================================================================================
    else if request.method == "DELETE" {
        response = delete_method(request, resources_path);

    }

    // ======================================================================================
    // In the case that the request is PUT
    // ======================================================================================
    else if request.method == "PUT" {
        response = put_method(request, resources_path);
    }

    // ======================================================================================
    // In the case that the request is POST
    // ======================================================================================
    else if request.method == "POST" {
        response = post_method(request, resources_path);
    }

    else {
        response = "".to_string();
    }

    println!("=====================================");
    println!("\n{}", response);
    println!("\n");

    unsafe {
        THREADS += 1;
    }
    stream.write(response.as_bytes()).unwrap();

}

// ======================================================================================
// The get method is declared
// Inputs: request_parser and resources_path
// Outputs: A string of characters
// ======================================================================================
fn get_method(request: request_parser::request::Request, resources_path: String) -> String {

    // ====================================================================================== 
    // Variables are declared
    // ====================================================================================== 
    let resource: String;
    let resource_data: String;
    let status: ResponseStatus;

    let not_found_resource_path = format!("{}/{}", resources_path, "not_found.html");
    let not_found_resource = fs::read_to_string(not_found_resource_path).unwrap();


    // ======================================================================================
    // Set a sleep between requests
    // ======================================================================================
    if request.resource == "/sleep" {
        resource = format!("{}/{}", resources_path, "index.html");  
        thread::sleep(Duration::from_secs(10));
    }
    
    else {
        resource = format!("{}{}", resources_path, request.resource);
    }

    let resource_data_result = fs::read_to_string(resource);

    // ======================================================================================
    // In the event that the request is found
    // ======================================================================================
    match resource_data_result {

        Ok(data) => {
            status = ResponseStatus::OK;
            resource_data = data;
        }

        Err(_) => {
            status = ResponseStatus::NotFound;
            resource_data = not_found_resource;
        }
    };

    // ======================================================================================
    // get state is set
    // ======================================================================================
    let status_line = format!("HTTP/1.1 {} {}", status.get_code(), status.get_message());
    
    // ======================================================================================
    // The response is generated
    // ======================================================================================
    let response = response::generate_response(status_line, resource_data.len(), resource_data);

    return response;
}

// ======================================================================================
// The get method is declared
// Inputs: request_parser and resources_path
// Outputs: A string of characters
// ======================================================================================
fn head_method(request: request_parser::request::Request, resources_path: String) -> String {

    // ====================================================================================== 
    // Variables are declared
    // ====================================================================================== 
    let status: ResponseStatus;
    let resource = format!("{}{}", resources_path, request.resource);
    let resource_data_result = fs::read_to_string(resource);

    // ======================================================================================
    // In the event that the request is found
    // ======================================================================================
    match resource_data_result {

        Ok(_data) => {
            status = ResponseStatus::OK;
        }

        Err(_) => {
            status = ResponseStatus::NotFound;
        }
    };

    // ======================================================================================
    // get state is set
    // ======================================================================================
    let status_line = format!("HTTP/1.1 {} {}", status.get_code(), status.get_message());

    // ======================================================================================
    // The response is generated
    // ======================================================================================
    let response = response::generate_response(status_line, 0, "".to_string());

    return response;
}


// ======================================================================================
// The get method is declared
// Inputs: request_parser and resources_path
// Outputs: A string of characters
// ======================================================================================
fn delete_method(request: request_parser::request::Request, resources_path: String) -> String {

    // ====================================================================================== 
    // Variables are declared
    // ====================================================================================== 
    let status: ResponseStatus;
    let resource = format!("{}{}", resources_path, request.resource);
    let resource_data_result = fs::read_to_string(resource);

    // ======================================================================================
    // In the event that the request is found
    // ======================================================================================
    match resource_data_result {

        Ok(_data) => {
            status = ResponseStatus::OK;
            remove_file(format!("{}{}", resources_path, request.resource)).unwrap();
        }

        Err(_) => {
            status = ResponseStatus::NotFound;
        }
    };

    // ======================================================================================
    // get state is set
    // ======================================================================================
    let status_line = format!("HTTP/1.1 {} {}", status.get_code(), status.get_message());

    // ======================================================================================
    // The response is generated
    // ======================================================================================
    let response = response::generate_response(status_line, 0, "".to_string());

    return response;
}


// ======================================================================================
// The get method is declared
// Inputs: request_parser and resources_path
// Outputs: A string of characters
// ======================================================================================
fn put_method(request: request_parser::request::Request, resources_path: String) -> String {

    // ====================================================================================== 
    // Variables are declared
    // ====================================================================================== 
    let status: ResponseStatus;
    let mut split_resource: Vec<&str> = request.resource.split("/").collect();
    let resource_name = split_resource.pop().unwrap().to_string();


    let resource = format!("{}/{}", resources_path, resource_name);


    let resource_data_result = File::open(resource);

    // ======================================================================================
    // In the event that the request is found
    // ======================================================================================
    match resource_data_result {

        Ok(_data) => {
            status = ResponseStatus::OK;
            fs::copy(request.resource.clone(), format!("{}/{}", resources_path, resource_name)).unwrap();
        }

        Err(_) => {
            status = ResponseStatus::NotFound;
        }
    };

    // ======================================================================================
    // get state is set
    // ======================================================================================
    let status_line = format!("HTTP/1.1 {} {}", status.get_code(), status.get_message());

    // ======================================================================================
    // The response is generated
    // ======================================================================================
    let response = response::generate_response(status_line, 0, "".to_string());
    return response;
}


fn post_method(request: request_parser::request::Request, resources_path: String) -> String {

    // ====================================================================================== 
    // Variables are declared
    // ====================================================================================== 
    let status: ResponseStatus;
    let mut split_resource: Vec<&str> = request.resource.split("/").collect();
    let resource_name = split_resource.pop().unwrap().to_string();


    let resource = format!("{}/{}", resources_path, resource_name);


    let resource_data_result = File::create(resource);

    // ======================================================================================
    // In the event that the request is found
    // ======================================================================================
    match resource_data_result {

        Ok(_data) => {
            status = ResponseStatus::OK;
            fs::copy(request.resource.clone(), format!("{}/{}", resources_path, resource_name)).unwrap();
        }

        Err(_) => {
            status = ResponseStatus::NotFound;
        }
    };

    // ======================================================================================
    // get state is set
    // ======================================================================================
    let status_line = format!("HTTP/1.1 {} {}", status.get_code(), status.get_message());
    
    // ======================================================================================
    // The response is generated
    // ======================================================================================
    let response = response::generate_response(status_line, 0, "".to_string());
    return response;
}