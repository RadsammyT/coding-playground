#include <iostream>
#include <vector>
using namespace std;
int main(int, char**) {
	int a[1] = {1234};
	for (int i = 0; i < 100; i++) {
		printf("%p : %d \n", &a[i], a[i]);
	}

	return 0;
}
