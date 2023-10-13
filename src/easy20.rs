// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
 

// Example 1:

// Input: s = "()"
// Output: true
// Example 2:

// Input: s = "()[]{}"
// Output: true
// Example 3:

// Input: s = "(]"
// Output: false
 

// Constraints:

// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.
use std::collections::{HashMap, VecDeque};

pub struct Solution {

}


impl Solution {
    pub fn is_valid(s: String) -> bool {
        let hash_map: HashMap<String, String> = HashMap::from([
            (")".to_string(),"(".to_string()),
            ("]".to_string(),"[".to_string()),
            ("}".to_string(),"{".to_string())
        ]);
        let mut stack: VecDeque<String> =  VecDeque::from([]);
        for  char in s.chars() {
            let is_should_continue = match hash_map.get(&char.to_string()) {
                Some(x) => Self::handle_close_parenthese(x, stack.pop_back()),
                None => { stack.push_back(char.to_string()); true}
            };

            if !is_should_continue {
                return false; 
            }
            //attempted to pop but stack doesn't remain element.
            // in the end elements remain in stack.
            // parenthese not match

        }

        if stack.len() > 0 {
            return false;
        }

        return true;
    }

    fn handle_close_parenthese(current_parenthese: &String, for_compare_parenthese: Option<String>) -> bool {
        match for_compare_parenthese {
            Some(parenthese) => return parenthese == *current_parenthese,
            None => return false,
        };
    }

}


// this solution beats 100% with runtime 0ms but 14% memory allocation
// the way to improve memory is remove hash_map and add more conditon in loop of char