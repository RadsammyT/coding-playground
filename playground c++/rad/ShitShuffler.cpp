#include <iostream>
#include <stdio.h>
#include <vector>
#include "timer.h"
#include <stdlib.h>
#include "input.cpp"
#pragma once
using namespace std;
namespace ShitShuffler {
    void printVector(vector < int > v);
    void randomizeVector(vector < int > & v);
    bool isUnique(vector < int > v);

    void printVector(vector < int > v) {
        // print array
        for (int i = 0; i < v.size(); ++i) {
            std::cout << v[i] << " ";
        }
        return;
    }
    void randomizeVector(vector < int > & v) {
        //srand(time(NULL));

        for (int i = 0; i < v.size(); i++) {
            v[i] = rand() % (v.size()) + 1;
        }

        return;
    }

    bool isUnique(vector < int > v) {
        for (int i = 0; i < v.size(); i++) {
            for (int j = i + 1; j < v.size(); j++) {
                if (v[i] == v[j] && i != j) {
                    return false;
                }
            }
        }

        return true;
    }
    // TIL that the order in which a function is initialized does matter when
    // calling it in a main function
    void run(int length, int max) {

        int failMark = 0;
        int failStep = 5000;
        int repeat = 0;
        long fail = 0;
        std::vector < int > v;
        std::vector<float> vt;
        timer::Timer per;
        
        srand(time(NULL));
        
        while (repeat <= max - 1) {
            if(!per.isStarted) {
                per.startTimer();
            }
            v.clear();
            v.resize(length);
            randomizeVector(v);
            if (isUnique(v)) {
                per.endTimer();
                vt.push_back(per.getTime());
                printVector(v);
                printf(" : %d, %.3f \n", fail, per.getTime());
                repeat++;
            } else {
                fail++;
                if (fail >= failMark) {
                    failMark = failMark + failStep;
                    printf("fail: %d\r", fail);
                }
            }
        }
        float vtSum = 0;
        for(int i = 0; i < vt.size(); i++) {
            vtSum += vt[i];
        }
        printf("TOTAL: %.3f \n", vtSum);

        input::userHalt();
    }

    int runNoParam() {
        int length = 0;
        int max = 0;
        while (length < 1) {
            length = input::readInt("Length?:");
        }
        while (max < 1) {
            max = input::readInt("Max?:");
        }
        ShitShuffler::run(length, max);
        return 0;
    }
}