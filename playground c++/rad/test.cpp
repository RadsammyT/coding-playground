#include <iostream>
#include <cstring>
#include <vector>
#include "test.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "input.cpp"
#include "pointers.cpp"
#pragma once
namespace test {
    int error() {
        // rad::testClass t;
        // t.setT(43);
        // printf("%d\n", t.getT());
        for(int i = 0; i < 130 + 1; i++) {
            if(strerror(i) != "Unknown Error") {
                printf("%d:  %s \n", i, strerror(i));
                
            }
        }

        return 0;
    }
    
    int floats() {
        auto x = input::readF12("F12: ");
        auto y = input::readF8("F8: ");
        auto z = input::readF4("F4: ");
        printf("%Lf \n", x);
        printf("%f \n", y);
        printf("%f \n", z);

        return 0;
    }

    int arrayPointers() {
        int arr[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
        pointers::arrayPointer(&arr[0], 10);

        return 0;
    }

}

