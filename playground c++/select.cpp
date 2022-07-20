#include "rad/ShitShuffler.cpp"
#include "rad/input.cpp"
#include "rad/pointers.cpp"
#include "rad/collatz.cpp"
int main() {
    int select = input::readI32("1: ShitShuffler \n2: test print lmao \n");
    

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
        default:
            printf("Invalid selection, not in switch list");
            break;
    }
}