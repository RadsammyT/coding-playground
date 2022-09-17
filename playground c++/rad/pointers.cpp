#include <iostream>
#include <cmath>
#pragma once
namespace pointers {
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

    int arrayPointer(int* zerothIndex, int length) {
        for (int i = 0; i < length; i++ ) {
            printf("%d \n", *(zerothIndex + i));
        }

        return 0;
    }

    int printp(auto* p) {
        printf("%p: %d", p, *p);

        return 0;
    }
}