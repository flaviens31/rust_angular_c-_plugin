use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use ffi::{add, print_message};
use serde::Serialize;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("plugin.h");

        // Define the C++ DataModel type for FFI
        type DataModel;

        // Declare the new_data_model function from C++ which creates a new DataModel
        fn new_data_model(id: i32) -> UniquePtr<DataModel>;

        // Declare the add function from C++ which adds two integers
        fn add(a: i32, b: i32) -> i32;

        // Declare the print_message function from C++ which prints a message to the console
        fn print_message();

        // Declare the calculate method from C++ which performs a calculation on the DataModel
        fn calculate(&self) -> i32;
    }

    // Define a struct in Rust that mirrors the C++ DataModel
    #[derive(Debug, Serialize, Deserialize)]
    struct DataModel {
        id: i32,
    }
}

#[get("/api/add/{a}/{b}")]
async fn add_numbers(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = add(a, b); // Call the C++ add function
    HttpResponse::Ok().json(result) // Return the result as JSON
}

#[derive(Serialize)]
struct DataModelResponse {
    id: i32,
    name: String,
}

#[get("/api/data-model/{id}")]
async fn get_data_model(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    // Create a new DataModel using the C++ function
    let mut model: cxx::UniquePtr<ffi::DataModel> = ffi::new_data_model(id);

    // Call the calculate method on the DataModel and update the id
    model.id = model.calculate();

    // Create a response with the updated DataModel
    let response = DataModelResponse {
        id: model.id,
        name: "Test".to_string(), // Placeholder name since the name field was removed
    };

    HttpResponse::Ok().json(response) // Return the response as JSON
}

#[get("/api/message")]
async fn message() -> impl Responder {
    print_message(); // Call the C++ function to print a message

    HttpResponse::Ok().json("Message printed to console") // Return a success message
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(add_numbers) // Register the add_numbers endpoint
            .service(get_data_model) // Register the get_data_model endpoint
            .service(message) // Register the message endpoint
    })
    .bind("127.0.0.1:8081")? // Bind the server to the local address and port 8081
    .run()
    .await
}
