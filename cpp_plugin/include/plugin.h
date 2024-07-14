#ifndef PLUGIN_H
#define PLUGIN_H

#include "rust/cxx.h"
#include <memory>
#include <string>

// Definition of the DataModel struct
struct DataModel {
    int id;             // An integer to hold the ID of the data model
    std::string name;   // A string to hold the name of the data model

    // Method to perform a calculation on the DataModel
    int calculate() const;
};

// Function to create a new DataModel instance
std::unique_ptr<DataModel> new_data_model(int id);
// std::unique_ptr<DataModel> new_data_model(int id,const std::string& name);

// Function to add two integers
int add(int a, int b);

// Function to print a message to the console
void print_message();

#endif // PLUGIN_H
