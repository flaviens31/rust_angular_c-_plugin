fn main() {
    // Use cxx_build to handle the C++ and Rust bridge.
    cxx_build::bridge("src/main.rs")
        // Specify the C++ source file to be compiled.
        .file("../cpp_plugin/src/plugin.cpp")
        // Include the directory where the C++ header files are located.
        .include("../cpp_plugin/include")
        // Use the C++14 standard if supported by the compiler.
        .flag_if_supported("-std=c++14")
        // Compile the C++ and Rust bridge, outputting an object file named "cxxbridge-demo".
        .compile("cxxbridge-demo");

    // These println! macros inform Cargo to re-run the build script if any of these files change.
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=../cpp_plugin/src/plugin.cpp");
    println!("cargo:rerun-if-changed=../cpp_plugin/include/plugin.h");
}
