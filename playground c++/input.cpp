// I can't convert this into a header file
// it would error otherwise but idfk aaaaa

#include <string>
#include <iostream>
namespace rad { 
    int readInt(std::string prompt) {
        int x;
        std::cout << prompt;
        try {
            std::cin >> x;
        } catch(std::exception e) {
            std::cout << "Invalid input" << std::endl;
            return readInt(prompt);
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

    void userHalt(std::string message = "") {
        std::cout << message;
        std::cin.ignore();
        std::cin.get();
    }

}