#include <iostream>
#include <vector>
#include "rad/timer.h"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
using namespace std;

const int A = 1234;
int main() {
	int allah = 124;
	int* test = &allah;
	
	int* arr[] = {(int*) &test};
	printf("const @ %p as %d\n", &test, test);
	for (int i = -10; i < 10; i++) {
		printf("%p (%d) \n", arr[i], i);
	}

	pointers::printp(test);
	return 0;
}

