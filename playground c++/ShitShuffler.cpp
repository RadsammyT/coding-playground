#include <random>
#include <iostream>
#include <cstring>
#include <vector>

/**
 * @brief 
 * 
 */

void printArray();
void randomizeArray();
void printArray(int arrIn[])
{
    // print array
    for (int i = 0; i < sizeof(arrIn) / sizeof(arrIn[0]); ++i)
    {
        std::cout << arrIn[i] << " ";
    }
    printf("\n");
    return;
}

void randomizeArray(int arrIn[])
{
    srand(time(NULL));

    for (int i = 0; i < sizeof(arrIn) / sizeof(arrIn[0]); i++)
    {
        arrIn[i] = rand() % (sizeof(arrIn) / sizeof(arrIn[0])) + 1;
    }

    return;
}

// checks if the int array has no duplicate numbers
bool isUnique(int arrIn[])
{
   /* 
   int target = sizeof(arr) / sizeof(arr[0]);
    short int count = 0;
    for (int i = 0; i < sizeof(arr) / sizeof(arr[0]); i++)
    {
        if(arr[i] == target)
        {
            count++;
        }

    }

    if(count == 1 )
    {
        return true;
    }
    

    return false;
    */

   for (int i; i < sizeof(arrIn) / sizeof(arrIn[0]); i++)
   {
         for (int j; j < sizeof(arrIn) / sizeof(arrIn[0]); j++)
         {
              if(i != j)
              {
                if(arrIn[i] == arrIn[j])
                {
                     return false;
                }
              }
         }
   }

       return true;
}



int main()
{

    srand(time(NULL));

    int max, length;
    std::cout << "Enter the max number: ";
    std::cin >> max;

    std::cout << "Enter the length of the array: ";
    std::cin >> length;

    int arr[length];

    for(int i = 0; i < length; i++)
    {
        arr[i] = rand() % max + 1;
    }

    while(!isUnique(arr))
    {
        randomizeArray(arr);

    }
    for(int i = 0; i < length; i++)
    {
        std::cout << arr[i] << " ";
    }


}