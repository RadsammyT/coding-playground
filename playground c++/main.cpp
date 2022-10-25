#include <iostream>
#include <vector>
#include "rad/timer.h"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
using namespace std;

const int A = 1234;
const int *testConst = &A;
int main() {
	int allah = A;
	// int* test = &A;
	
	int* arr[] = {(int*) &testConst};
	printf("const @ %p as %d\n", &testConst, testConst);
	for (int i = -10; i < 10; i++) {
		printf("%p (%d) \n", arr[i], i);
	}

	return 0;
}

