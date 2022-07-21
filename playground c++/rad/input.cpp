// I can't convert this into a header file
// it would error otherwise but idfk aaaaa

#include <string>
#include <iostream>
#pragma once
namespace input { 
    int readI32(std::string prompt) {
        int x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readI32(prompt);
        }
        return x;
    }

    std::string readString(std::string prompt) {
        std::string x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readString(prompt);
        }

        return x;
    }

    long long readI64(std::string prompt) {
        long long x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readI64(prompt);
        }
        return x;
    }

    void userHalt(std::string message = "") {
        std::cout << message;
        std::cin.ignore();
        std::cin.get();
    }

}