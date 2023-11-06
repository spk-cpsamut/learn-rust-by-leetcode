
// Given an array nums of size n, return the majority element.

// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

 

// Example 1:

// Input: nums = [3,2,3]
// Output: 3
// Example 2:

// Input: nums = [2,2,1,1,1,2,2]
// Output: 2
 

// Constraints:

// n == nums.length
// 1 <= n <= 5 * 104
// -109 <= nums[i] <= 109
 

// Follow-up: Could you solve the problem in linear time and in O(1) space?


use std::collections::HashMap;

pub struct Solution {

}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut accumulator: HashMap<&i32, usize> = HashMap::new();
        let majority_measurement = nums.len() / 2;
        for num in nums.iter()  {
            match  accumulator.get(num) {
                Some(n) => {
                    if n >= &majority_measurement {
                        return *num;
                    } else {
                        accumulator.insert(num,n+1);
                    }
                },
                None => {
                    accumulator.insert(num, 1);
                },
            }
        }

        return nums[0];
    }

    pub fn run_test_cases() {
        // assert_eq!(self::Solution::majority_element([3,2,3].to_vec()), 3);
        // assert_eq!(self::Solution::majority_element([2,2,1,1,1,2,2].to_vec()), 2);
        assert_eq!(self::Solution::majority_element([5].to_vec()), 5);
    }
}