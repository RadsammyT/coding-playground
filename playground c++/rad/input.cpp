// I can't convert this into a header file
// it would error otherwise but idfk aaaaa

#include <string>
#include <iostream>
#include <cstdio>
#include <ios>
#pragma once
namespace input { 
    /**
     * @brief Returns an int based on user input
     * 
     * @param prompt 
     * @return int 
     */
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
    /**
     * @brief Returns a string based on user input until newline
     * 
     * @param prompt 
     * @return std::string 
     */
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
    /**
     * @brief returns a long long int based on user input
     * 
     * @param prompt 
     * @return long long int 
     */
    long long int readI64(std::string prompt) {
        long long int x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readI64(prompt);
        }

        return x;
    }
    /**
     * @brief readFXX, where XX = byte size
     * 
     * @param prompt 
     * @return float 
     */
    float readF4(std::string prompt) {
        float x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readF4(prompt);
        }
        return x;
    }

    /**
     * @brief readFXX, where XX = byte size
     * 
     * @param prompt 
     * @return double 
     */
    double readF8(std::string prompt) {
        double x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readF8(prompt);
        }
        return x;
    }
    /**
     * @brief readFXX, where XX = byte size
     * 
     * @param prompt 
     * @return long double 
     */
    long double readF12(std::string prompt) {
        long double x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readF12(prompt);
        }
        return x;
    }
    
    /**
     * @brief Halts the running program on invocation until user pushes enter/return
     * 
     * @param message 
     */
    void userHalt(std::string message = "") {
        std::cout << message;
        std::cin.ignore();
        std::cin.get();
    }



}