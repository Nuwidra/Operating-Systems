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

use hyper::{Client, Body, Method, Request};
//use hyper::body;
//use std::env;

// ======================================================================================
// Function in the case that the client selects a GET
// ======================================================================================
pub async fn get(direction: &str, resource: &str) {

    let resource_full_path = format!("{}{}", direction, resource);

    // ======================================================================================
    // The request is built
    // ======================================================================================
    let request = Request::builder()
        .method(Method::GET)
        .uri(resource_full_path)
        .header("accept", "application/json")
        .body(Body::from(resource.to_string())).unwrap();

    // ======================================================================================
    // A new customer is created
    // ======================================================================================
    let client = Client::new();
    
    // ======================================================================================
    // The request is processed
    // ======================================================================================
    client.request(request).await.unwrap();
    //println!("Response GET: {}", resp.status());
    //let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );

}

// ======================================================================================
// Function in the case that the client selects a HEAD
// ======================================================================================
pub async fn head(direction: &str, resource: &str) {

    let resource_full_path = format!("{}{}", direction, resource);

    // ======================================================================================
    // The request is built\
    // ======================================================================================
    let request = Request::builder()
        .method(Method::HEAD)
        .uri(resource_full_path)
        .header("accept", "application/json")
        .body(Body::from(resource.to_string())).unwrap();

    // ======================================================================================
    // A new customer is created
    // ======================================================================================
    let client = Client::new();

    // ======================================================================================
    // The request is processed
    // ======================================================================================
    client.request(request).await.unwrap();
    //println!("Response GET: {}", resp.status());
    //let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );

}

// ======================================================================================
// Function in the case that the client selects a DELETE
// ======================================================================================
pub async fn delete(direction: &str, resource: &str) {

    let resource_full_path = format!("{}{}", direction, resource);

    // ======================================================================================
    // The request is built
    // ======================================================================================
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(resource_full_path)
        .header("accept", "application/json")
        .body(Body::from(resource.to_string())).unwrap();

    // ======================================================================================
    // A new customer is created
    // ======================================================================================
    let client = Client::new();

    // ======================================================================================
    // The request is processed
    // ======================================================================================
    client.request(request).await.unwrap();
    //println!("Response GET: {}", resp.status());
    //let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );

}

// ======================================================================================
// Function in the case that the client selects a PUT
// ======================================================================================
pub async fn put(direction: &str, resource: &str) {
    
    let resource_full_path = format!("{}{}", direction, resource);

    // ======================================================================================
    // The request is built
    // ======================================================================================
    let request = Request::builder()
        .method(Method::PUT)
        .uri(resource_full_path)
        .header("accept", "application/json")
        .body(Body::from(resource.to_string())).unwrap();

    // ======================================================================================
    // A new customer is created
    // ======================================================================================
    let client = Client::new();

    // ======================================================================================
    // The request is processed
    // ======================================================================================
    client.request(request).await.unwrap();
    //println!("Response GET: {}", resp.status());
    //let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
}

// ======================================================================================
// Function in the case that the client selects a POST
// ======================================================================================
pub async fn post(direction: &str, resource: &str) {

    let resource_full_path = format!("{}{}", direction, resource);

    // ======================================================================================
    // The request is built
    // ======================================================================================
    let request = Request::builder()
        .method(Method::POST)
        .uri(resource_full_path)
        .header("accept", "application/json")
        .body(Body::from(resource.to_string())).unwrap();

    // ======================================================================================
    // A new customer is created
    // ======================================================================================
    let client = Client::new();

    // ======================================================================================
    // The request is processed
    // ======================================================================================
    client.request(request).await.unwrap();
    //println!("Response GET: {}", resp.status());
    //let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
}