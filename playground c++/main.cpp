#include <iostream>
#include <vector>
using namespace std;
int main(int, char**) {
	printf("test");
	vector<int> v;

	for (int i = 0; i < 10; i++) {
		v.push_back(i);
	}
	return 0;
    
}
