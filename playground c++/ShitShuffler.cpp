#include <iostream>
#include <stdio.h>
#include <vector>
#include "timer.h"
using namespace std;

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
/**
 * @brief test
 * 
 * @return int 
 */
int main() {

    int length = 0;
    int max = 0;
    while (length < 1) {
        printf("length?: ");
        cin >> length;
    }
    while (max < 1) {
        printf("max?: ");
        cin >> max;
    }
    int repeat = 0;
    int fail = 0;
    //vector methods : what they do
    //push_back : add an element to the end of the vector
    //pop_back : remove the last element of the vector
    //reserve : reserve specified amount of space for the vector
    //resize : resize the vector to specified size
    //clear : clear the vector along with all its elements
    //size : return the size of the vector
    //at : return the element at specified index
    //front : return the first element of the vector
    //back : return the last element of the vector
    //insert : insert an element at specified index
    //erase : erase an element at specified index
    //swap : swap two elements at specified index
    // brought to you by github copilot
    

    srand(time(NULL));
    std::vector < int > v;
    
    int failMark = 0;
    int failStep = 5000;
    rad::Timer t;
    t.startTimer();
    while (repeat <= max - 1) {
        v.clear();
        v.resize(length);
        randomizeVector(v);
        if (isUnique(v)) {
            printVector(v);
            printf(" : %d \n", fail);
            repeat++;
        } else {
            fail++;
            if (fail >= failMark) {
                failMark = failMark + failStep;
                printf("fail: %d\r", fail);
            }
        }
    }
    t.endTimer();
    t.printTime();
    return 0;
}