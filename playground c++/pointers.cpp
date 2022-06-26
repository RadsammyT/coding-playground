#include <iostream>

int test(int* p) {
    return *p;
}

int main() {
    int x = 10;
    int *p = &x;
    std::cout << test(p) << std::endl;
}