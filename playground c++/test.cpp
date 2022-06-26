#include <iostream>
#include <cstring>
#include <vector>
#include "test.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // rad::testClass t;
    // t.setT(43);
    // printf("%d\n", t.getT());
    for(int i = 0; i < 130 + 1; i++) {
        if(strerror(i) != NULL) {
            printf("%d:  %s \n", i, strerror(i));
            
        }
    }
}

