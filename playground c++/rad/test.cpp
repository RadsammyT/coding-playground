#include <iostream>
#include <cstring>
#include <vector>
#include "test.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
namespace test {
    int test() {
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
}

