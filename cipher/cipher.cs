// https://www.hackerrank.com/challenges/cipher

using System;
using System.Text;
using System.Collections.Generic;
using System.IO;
class Solution {
    static void Main(String[] args) {
        string ciphertext;
        string plaintext;
        int n, k;
        int size;
        char nextchar;
        string[] parse = Console.ReadLine().Split(' ');
        n = int.Parse(parse[0]); k = int.Parse(parse[1]);
        size = n + k - 1;
        ciphertext = Console.ReadLine();
        plaintext = new string('x', n);
        
        int num_ones = ciphertext[size-1] - '0';
        int p_i = n - 2;
        unsafe 
        {
            fixed (char *cursor = plaintext)
            {
                cursor[n-1] = ciphertext[size-1];
                for (int i = size-2; i > k-2; i--) 
                {
                    if (ciphertext[i] == '1')
                    {
                        if (num_ones % 2 == 0)
                            nextchar = '1';
                        else
                            nextchar = '0';
                    }
                    else
                    {
                        if (num_ones % 2 == 0)
                            nextchar = '0';
                        else
                            nextchar = '1';
                    }
                    cursor[p_i] = nextchar;
                    num_ones += nextchar - '0' - ((n-p_i > k - 1) ? cursor[p_i + k - 1] - '0' : 0);
                    p_i--;
                }
            }
        }
        
        Console.WriteLine(plaintext);
    }
}