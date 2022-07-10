#include "rad/ShitShuffler.cpp"
#include "rad/input.cpp"

int main() {
    int select = input::readInt("1: ShitShuffler \n 2: test print lmao \n ");

    switch(select) {
        case 1:
            ShitShuffler::runNoParam();
        case 2:
            printf("test moment");
        default:
            printf("Invalid selection, not in switch list");
    }
}