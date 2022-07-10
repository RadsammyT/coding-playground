#include <iostream>
#include <vector>
#include "rad/timer.h"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
using namespace std;
int main() {
	int test = input::readInt("test int? ");
	printf("%d", test);
	return 0;
}
