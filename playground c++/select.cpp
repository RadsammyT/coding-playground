#include "rad/ShitShuffler.cpp"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
#include "rad/collatz.cpp"
#include <vector>

void fibb() {
    int len = input::readI32("how many numbers?: ");

    std::vector<int> res = {};
    res.push_back(0);
    res.push_back(1);
    for (int i = 0; i < len - 2; i++) {
        res.push_back(res.at(i) + res.at(i + 1));
    }

    for(auto i: res) {
        printf("%d ", i);
    }
}

int main() { 
    int select = input::readI32("1: ShitShuffler \n2: test print lmao \n3: collatz \n4: fibbonachi\n");

    switch(select) {
        case 1:
            ShitShuffler::runNoParam();
            break;
        case 2:
            printf("test moment");
            break;
        case 3:
            collatz::runInSwitch();
            break;
        case 4:
            fibb();
            break;
        default:
            printf("Invalid selection, not in switch list");
            break;
    }
}