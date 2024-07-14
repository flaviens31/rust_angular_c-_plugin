# Rust-C++ Integration with Actix Web and Angular

This project demonstrates the integration of Rust with C++ using `cxx`, and provides a REST API using `actix-web`. The frontend is built using Angular to consume the API.

## Project Structure

- **cpp_plugin**: Contains the C++ code.
- **rust_backend**: Contains the Rust backend using `actix-web`.
- **angular_frontend**: Contains the Angular frontend.

## Prerequisites

- Rust (latest stable version)
- Node.js and npm
- Angular CLI
- A C++ compiler (e.g., g++, clang)

## Setup Instructions

### 1. Setup C++ Plugin

Navigate to the `cpp_plugin` directory and build the C++ code.

```sh
cd cpp_plugin
mkdir build
cd build
cmake ..
make
```

### 2. Setup Rust Backend

Navigate to the rust_backend directory and build the Rust project.

```sh
cd rust_backend
cargo build
```

Ensure the Rust project is correctly set up to link against the C++ library by using cxx.

### 3. Setup Rust Backend

Navigate to the angular_frontend directory and install dependencies.

```sh
cd angular_frontend
npm install
```

Build and serve the Angular application.

```sh
ng serve
```

## Running the Project

### 1. Start the Rust server

```sh
cd rust_backend
cargo run
```

This will start the Rust server on 127.0.0.1:8081.

### 2. Start the Angular frontend

```sh
cd angular_frontend
ng serve
```

This will start the Angular application on <http://localhost:4200>.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](./LICENSE) file for details.
