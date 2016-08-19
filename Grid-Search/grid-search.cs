//https://www.hackerrank.com/challenges/the-grid-search

using System;
using System.Linq;
using System.Collections.Generic;
using System.IO;
class Solution {
    static int grid_rows, grid_cols;
    static int pattern_rows, pattern_cols;
    static List<int[]> grid, pattern;
    
    static void Main(String[] args) {
        int test_cases;
        
        
        test_cases = int.Parse(Console.ReadLine());
        
        for (int i = 0; i < test_cases; i++)
        {
            string[] parsedInput = Console.ReadLine().Split(' ');
            grid_rows = int.Parse(parsedInput[0]);
            grid_cols = int.Parse(parsedInput[1]);
            grid = new List<int[]>();
            for (int row = 0; row < grid_rows; row++)
                grid.Add(Console.ReadLine().Select(c => c - '0').ToArray());
            parsedInput = Console.ReadLine().Split('\t', ' ');
            pattern_rows = int.Parse(parsedInput[0]);
            pattern_cols = int.Parse(parsedInput[1]);
            pattern = new List<int[]>();
            for (int row = 0; row < pattern_rows; row++)
                pattern.Add(Console.ReadLine().Select(c => c - '0').ToArray());
            
            bool result = false;
            for (int row = 0; row < grid_rows && !result; row++)
            {
                for (int col = 0; col < grid_cols && !result; col++)
                {
                    result = InGrid(row, col);
                }
            }
            
            Console.WriteLine((result) ? "YES" : "NO");
        }
    }
    
    static bool InGrid(int row, int col)
    {
        if (grid_rows < (row+pattern_rows) ||
            grid_cols < (col+pattern_cols))
            return false;
        
        for (int irow = 0; irow < pattern_rows; irow++) {
            for (int icol = 0; icol < pattern_cols; icol++) {
                if (pattern[irow][icol] != grid[row+irow][col+icol])
                    return false;
            }
        }
        
        return true;
    }
}