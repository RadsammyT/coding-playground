#include <iostream>
#include <vector>
#include "timer.h"
#include "input.cpp"
using namespace std;
int main(int argc, char** argv) {
	rad::Timer t;
	t.startTimer();
	int a[1] = {1234};
	for (int i = 0; i < 100; i++) {
		printf("%p : %d \n", &a[i], a[i]);
	}

	t.endTimer();

	printf("%.3f \n", t.getTime());
	cout << readInt("Enter an integer: ") << "\n";
	cout << readString("String: ") << "\n";
	userHalt("Press enter to exit...");
	return 0;
}
