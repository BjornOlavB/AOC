
//Given an integer x, return true if x is a palindrome, and false otherwise.

//Constraints:
//-231 <= x <= 231 - 1
//Try doing it without stringifying the number

public class Solution
{
    public static bool IsPalindrome(int x)
    {
        List<int> listofX = new List<int>();
        if (x < 0){
            return false;
        }
        while (x > 0)
        {
            listofX.Add(x % 10);
            x = x / 10;
        }
        for (int b = 0; b < listofX.Count; b++)
        {
            if (listofX.First() == listofX.Last())
            {
                listofX.RemoveAt(listofX.Count-1);
                listofX.RemoveAt(0);

            }
        }
        if (listofX.Count == 0 || listofX.Count == 1)
        {
            Console.WriteLine("True");
            return true;
        }
        else
        {
            Console.WriteLine("False");
            return false;
        }
    }
}
class TestClass
{
    static void Main(string[] args)
    {
        int x = 72557;
        Solution.IsPalindrome(x);
    }
}