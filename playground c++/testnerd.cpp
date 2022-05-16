#include <iostream>
#include <vector>
#include <string>
#include <stdio.h>
#include <array>

using namespace std;
bool isUnique(int arr[])
{
    // if all the numbers aren't repeating, then it's unique
    // if there are repeating numbers, then it's not unique
    // so we can use a bit vector to check if there are repeating numbers
    for (int i = 0; i < sizeof(arr) / sizeof(arr[0]); i++)
    {
        for (int j = 0; j < sizeof(arr) / sizeof(arr[0]); j++)
        {
            printf("%d %d\n", i, j);
            // printf here because it will always return true when it shouldn't in one test case
            // solution? 

            
            if (arr[i] == arr[j]&& i - j != 0) // wtf
            {
                printf("fail, indices %d %d are %d %d", i, j, arr[i], arr[j]);
                return false;
            }
        }
    }

       return true;
}

int main()
{
    srand(time(NULL));

    int len;
    cout << "length?: "; cin >> len;
    int max;
    cout << "max? "; cin >> max;
    int array[len];
    int repeat = 0;
    int fail;

//while(repeat <= max){
    while(!isUnique(array))
    {
        for (int i; i < sizeof(array)/sizeof(array[0]); i++)
        {
            array[i] = rand() % len + 1;
        }
        fail++;
        cout << "fail: " << fail << "                        \r";
        _sleep(500);
    }
//}


    for (int i; i < sizeof(array)/sizeof(array[0]); i++)
    {
        cout << array[i] << " ";
    }

    cout << endl;
    cout << "fail: " << fail << endl;

    

        return 0;
}
