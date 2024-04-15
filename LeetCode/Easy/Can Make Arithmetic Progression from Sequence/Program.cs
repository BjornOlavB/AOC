//A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.
//Given an array of numbers arr, return true if the array can be rearranged to form an arithmetic progression. Otherwise, return false.

//Constraints:
//2 <= arr.length <= 1000
//-106 <= arr[i] <= 106

public class Solution
{
    public static bool arithmeticCheck(int[] arr)
    {
        if (arr.Length == 2)
            return true;
        Array.Sort(arr);
        int difference = arr[1] - arr[0];
        for (int i = 2; i < arr.Length; i++)
        {
            if (arr[i] != (arr[i - 1] + difference))
                return false;
        }
        Console.WriteLine("Array is Arithmetic");
        return true;
    }
}
class TestClass
{
    static void Main(string[] args)
    {
        // Display the number of command line arguments.
        int[] myArray = {1,2,3,4,5,6,7,8,9,10};
        Solution.arithmeticCheck(myArray);
    }
}