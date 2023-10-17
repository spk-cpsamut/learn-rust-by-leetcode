// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.

 

// Example 1:

// Input: nums = [1,3,5,6], target = 5
// Output: 2
// Example 2:

// Input: nums = [1,3,5,6], target = 2
// Output: 1
// Example 3:

// Input: nums = [1,3,5,6], target = 7
// Output: 4
 

// Constraints:

// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums contains distinct values sorted in ascending order.
// -104 <= target <= 104

use core::num;

pub struct Solution {

}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut index: usize = 0;

        if nums[0] == target || nums[0] > target {
            return index as i32;
        }

        for i in 1..nums.len() {
            if nums[i] == target || nums[i] > target {
                index = i; 
                break;
            };
        }
        if index == 0 {
            index = nums.len();
        }
        return index as i32;
    }

    pub fn run_units_test_easy_35() {
        assert_eq!(self::Solution::search_insert([1,3,5,6].to_vec(), 5), 2);
        assert_eq!(self::Solution::search_insert([1,3,5,6].to_vec(), 2), 1);
        assert_eq!(self::Solution::search_insert([1,3,5,6].to_vec(), 7), 4);
        assert_eq!(self::Solution::search_insert([1,3,5,6].to_vec(), 6), 3);
    }
}