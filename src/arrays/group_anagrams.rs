use std::collections::HashMap;

/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.


Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]

Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Explanation:

There is no string in strs that can be rearranged to form "bat".
The strings "nat" and "tan" are anagrams as they can be rearranged to form each other.
The strings "ate", "eat", and "tea" are anagrams as they can be rearranged to form each other.
*/

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // each group key is the sorted characters of the string
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        // sort characters to create canonical key
        let mut key_chars: Vec<char> = s.chars().collect();
        key_chars.sort_unstable();
        let key: String = key_chars.into_iter().collect();

        groups.entry(key).or_default().push(s);
    }

    // collect the values directly; order of groups is unspecified
    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let test_case = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let mut expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        let mut result = group_anagrams(test_case);

        // sort inner groups and outer list for deterministic comparison
        for group in &mut expected {
            group.sort_unstable();
        }
        for group in &mut result {
            group.sort_unstable();
        }
        expected.sort();
        result.sort();

        assert_eq!(result, expected);
    }
}
