use std::collections::HashMap;

/*
Problem URL: https://leetcode.com/problems/longest-substring-without-repeating-characters/

Solution:
Using sliding window,

Map the latest index of chars in a Map

right pointer is part of iterator loop over the chars of string and
the left part is only updated if the current ptr is greater than set one

record the max when we find match in set
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set: HashMap<char, usize> = HashMap::new();
    let mut max_len = 0;
    let mut left_ptr: usize = 0;

    for (i, c) in s.chars().enumerate() {
        // println!("Index: {}, Character: {}", i, c);

        match set.get(&c) {
            Some(c_index) => {
                if max_len < (i - left_ptr) {
                    max_len = i - left_ptr;
                    println!("i: {}, left: {}, maxLen : {}", i, left_ptr, max_len);
                }
                if c_index >= &left_ptr {
                    left_ptr = c_index.to_owned() + 1;
                }
            }
            None => {}
        }
        set.insert(c, i);
    }

    if max_len < (s.len() - left_ptr) {
        max_len = s.len() - left_ptr
    }

    (max_len).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let test_str = String::from("abcdabc");
        assert_eq!(length_of_longest_substring(test_str), 4);
    }

    #[test]
    fn testcase_2() {
        let test_str = String::from("bbbbb");
        assert_eq!(length_of_longest_substring(test_str), 1);
    }

    #[test]
    fn testcase_3() {
        let test_str = String::from(" ");
        assert_eq!(length_of_longest_substring(test_str), 1);
    }

    #[test]
    fn testcase_4() {
        let test_str = String::from("abba");
        assert_eq!(length_of_longest_substring(test_str), 2);
    }
}
