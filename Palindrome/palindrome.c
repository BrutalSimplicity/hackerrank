// https://www.hackerrank.com/challenges/palindrome-index

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() {
    int test_cases, index, result;
    char test[20][100005];
    scanf("%d", &test_cases);
    for (index = 0; index < test_cases; index++)
        scanf("%s", test[index]);
    
    for (index = 0; index < test_cases; index++)
    {
        result = -1;
        for (int l = 0, r = strlen(test[index]) - 1; l < r; l++, r--)
        {
            if (test[index][l] != test[index][r])
            {
                if (test[index][l+1] == test[index][r])
                {
                    result = l;
                    for (int x = l+1, y = r; x < y; x++, y--)
                    {
                        if (test[index][x] != test[index][y])
                        {
                            result = r;
                            break;
                        }
                    }
                }
                else if (test[index][r-1] == test[index][l])
                {
                    result = r;
                    for (int x = l, y = r-1; x < y; x++, y--)
                    {
                        if (test[index][x] != test[index][y])
                        {
                            result = l;
                            break;
                        }
                    }
                }
                break;
            }
        }
        
        printf("%d\n", result);
    }
    return 0;
}