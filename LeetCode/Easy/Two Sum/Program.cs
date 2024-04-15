//Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//You may assume that each input would have exactly one solution, and you may not use the same element twice.
//You can return the answer in any order.

//Constraints:

//2 <= nums.length <= 104
//-109 <= nums[i] <= 109
//-109 <= target <= 109
//Only one valid answer exists.
 

public class Solution {

    public static int[]? TwoSum(int[] nums, int target) {
        Dictionary <int, int> storedValues = new Dictionary<int, int>(); 
        for (int i = 0; i < nums.Length; i++)
        { 
            int holder = target - nums[i];
            if (storedValues.ContainsKey(holder))
            {
                return new int[] {storedValues[holder],i};
            }else
            {                   
                storedValues[nums[i]] = i;
            }
        }
        return null;
    }
}
// this is O(n) since searching through hashes is O(1).

class TestClass
{
    static void Main(string[] args)
    {
        //Feel free to change values.
        int target = 200;
        int[] myArray = {1,2,3,4,5,6,7,8,9,10};
        Solution.TwoSum(myArray, target);
    }
}
