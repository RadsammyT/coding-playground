#include "rad/ShitShuffler.cpp"
#include "rad/input.cpp"

int main() {
    int select = input::readInt("1: ShitShuffler \n2: test print lmao \n");

    switch(select) {
        case 1:
            ShitShuffler::runNoParam();
            break;
        case 2:
            printf("test moment");
            break;
        default:
            printf("Invalid selection, not in switch list");
            break;
    }
}