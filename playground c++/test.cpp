#include <iostream>
#include <cstring>
#include <vector>



int main()
{
    std::cout << __cplusplus << std::endl;
    char test[] = "This is a char array. C++ = cursed \n ";
    std::cout << test;
    int repeat = 0;
    int len;
    std::cout << "length of list?: ";
    std::cin >> len;

    std::cout << "How many times to repeat?: ";
    std::cin >> repeat;
    // seed the random number generator
    srand(time(NULL));
    int intTest[len];


    for (int r = 0; r < repeat; r++){
        for (int i = 0; i < 10; i++)
        {
            // randomize every number in the array from 1 to the size of array
            intTest[i] = rand() % (sizeof(intTest) / sizeof(intTest[0])) + 1;
        }

    // print the array
        for (int j = 0; j < sizeof(intTest) / sizeof(intTest[0]); ++j) 
        {
            std::cout << intTest[j] << " ";
        }
           
         printf("\n");
    }

    return 0;
}

