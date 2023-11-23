// Given a sorted list of numbers, remove duplicates and return the new length. You must do this in-place and without using extra memory.

// Input: [0, 0, 1, 1, 1, 2, 2].

// Output: 3.

// Your function should modify the list in place so the first 3 elements becomes 0, 1, 2. Return 3 because the new length is 3.


pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow: usize = 0;
        for fast in 0..nums.len() -1 {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }

        return (slow + 1) as i32;
    }

    pub fn run_test_cases() {
        assert_eq!(self::Solution::remove_duplicates(&mut [0, 0, 1, 1, 1, 2, 2].to_vec()), 3)
    }
}