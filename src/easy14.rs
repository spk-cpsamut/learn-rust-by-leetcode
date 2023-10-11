// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

 

// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.

pub struct Solution {

}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {  
        let mut longest_common_prefix: String = "".to_owned();
        let mut index: usize = 0;
        Self::add_prefix(strs, &mut index, &mut longest_common_prefix);
        return longest_common_prefix;
    }

    fn add_prefix(strs: Vec<String>, index: &mut usize, longest_common_prefix: &mut String) {
        let binding = strs[0].get(*index..*index+1);
        let  checker_slice = match &binding {
            Some(x) =>  x,
            None => return (),
        };

        for str in &strs {
            match str.get(*index..*index+1) {
                Some(x) if x != *checker_slice => return (),
                Some(x) => (),
                None => return (),
            }
        }
        *index += 1;
        longest_common_prefix.push_str(checker_slice);
        Self::add_prefix(strs, index, longest_common_prefix);
    }
}