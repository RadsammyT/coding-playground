#include <iostream>
#include <vector>
#include "rad/timer.h"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
using namespace std;

int test();

const int A = 1234;
const int *testConst = &A;
int main() {
	test();

	return 0;
}

int test() {
	throw runtime_error("allahu akabr");
}
