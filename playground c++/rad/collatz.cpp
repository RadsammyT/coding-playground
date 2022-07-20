#include <iostream>
#include "input.cpp"
#pragma once
namespace collatz {
    int* run(long long int input, bool printSteps)  {
        static int iter[] = {0, 0, 0};
        // iter, even, odd

        do
        {
            if(input % 2 == 1) {
                input = (3 * input) + 1;
                iter[0]++;
                iter[2]++;
                if(printSteps)
                    printf("%lld \n", input);
                continue;
            }

            if(input % 2 == 0) {
                input /= 2;
                iter[0]++;
                iter[1]++;
                if(printSteps)
                    printf("%lld \n", input);
                continue;
            }

        } while (input != 1);

        // printf("TEST\n");
        // for(int i: iter) {
        //     printf("%d ", i);
        // }
        // printf("\n");
        
        return &iter[0];
        
    }
    /**
     * @brief Can't return an array, 
     * 
     * @return int*, or specifically a pointer to the first element in the array. 
     */
    int* runNoParam() {
        long long int input = input::readI32("input?: ");
        return run(input, true);
    }
    /**
     * @brief because using runNoParam() along with initializing a var would throw an error
     * 
     */
    void runInSwitch() {
        int *out = runNoParam();
        int result[] = {*out, *(out + 1), *(out + 2)};
        for(int i: result){
            printf("%d ", i);
        }
        printf("\n");
    }
}