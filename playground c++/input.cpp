#include <string>
#include <iostream>

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
