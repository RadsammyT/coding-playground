#include <iostream>
#include <vector>
#include "rad/timer.h"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
using namespace std;
int main() {
	int test = input::readI32("test int? ");
	printf("%d", test);
	return 0;
}
