#include <iostream>
#include <cmath>
int test(int* p) {
    return *p;
}

int test2() {
    int x = 10;
    int* p = &x;
    std::cout << test(p) << "\n";

    int arr[] = {69};
    int f = 200 + 1;
    for (int i = -f; i < f; i++){
        printf("%p (%d): %d\n", &arr[i], i, arr[i]);
    }

    return 0;
}