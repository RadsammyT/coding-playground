#include <iostream>
#include <vector>
#include "rad/timer.h"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
using namespace std;

const int A = 1234;
int main() {
	
	int* arr[] = {(int*) &A};
	printf("const @ %p as %d\n", & A, A);
	for (int i = -10; i < 10; i++) {
		printf("%p (%d) \n", arr[i], i);
	}
		return 0;
}
