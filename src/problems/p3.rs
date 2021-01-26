/*
Given a string s, find the length of the longest substring without repeating characters.
*/

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    // Todo: Make it more efficient.

    pub fn length_of_longest_substring(s: String) -> i32 {
        fn get_max_length(chars_array: &Vec<char>) -> i32 {
            if chars_array.len() < 2 { return chars_array.len() as i32; }
            let mut found_chars: HashSet<&char> = HashSet::new();
            found_chars.insert(&chars_array[0]);
            let mut length = 1;
            let mut i = 1;
            while i < chars_array.len() {
                if chars_array[i] == chars_array[i - 1] { break; }
                if !found_chars.insert(&chars_array[i]) { break; }
                length += 1;
                i += 1;
            }
            return length;
        }

        let chars_array: Vec<char> = s.chars().into_iter().collect();
        let mut max_length = 0;
        for i in 0..chars_array.len() {
            let chopped_aray: Vec<char> = chars_array.iter().skip(i).map(|chr| *chr).collect();
            let lengh = get_max_length(&chopped_aray);
            if lengh > max_length { max_length = lengh; }
        }

        return max_length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let s = "abcabcbb";
        assert_eq!(Solution::length_of_longest_substring(s.to_string()), 3);

        let s = "au";
        assert_eq!(Solution::length_of_longest_substring(s.to_string()), 2);

        let s = "bbbbb";
        assert_eq!(Solution::length_of_longest_substring(s.to_string()), 1);

        let s = "pwwkew";
        assert_eq!(Solution::length_of_longest_substring(s.to_string()), 3);
    }
}