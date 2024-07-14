#include "plugin.h" // Include the header file with declarations
#include <iostream> // Include the input/output stream library for printing

// Implementation of the calculate method for the DataModel struct
int DataModel::calculate() const {
    return id * 5; // Perform a calculation using the id member and return the result
}

// Function to create a new DataModel instance with a given ID
std::unique_ptr<DataModel> new_data_model(int id) {
    // Use make_unique to create a new DataModel instance and return a unique pointer to it
    return std::make_unique<DataModel>(DataModel{id, "test"}); // Initialize with id and default name "test"
}

/* Function to create a new DataModel instance with a given ID and name
std::unique_ptr<DataModel> new_data_model(int id, const std::string& name) {
    return std::make_unique<DataModel>(DataModel{id, name}); // Initialize with id and provided name
}*/

// Function to add two integers and return the result
int add(int a, int b) {
    return a + b;
}

// Function to print a message to the console
void print_message() {
    std::cout << "Hello from C++!" << std::endl; // Output a message to the console
}
