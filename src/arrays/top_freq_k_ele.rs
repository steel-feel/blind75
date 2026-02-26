use std::collections::HashMap;

fn top_k_freq_ele(nums: Vec<i32>, k: u32) -> Vec<i32> {
    let mut result = vec![];

    let mut bucket: HashMap<i32, u32> = HashMap::new();

    for num in nums {
        match bucket.get(&num) {
            Some(v) => {
                bucket.insert(num, v + 1);
            }
            None => {
                bucket.insert(num, 1);
            }
        }
    }

    let mut folded: Vec<(i32, u32)> = bucket.into_iter().collect();

    folded.sort_by(|a, b| b.1.cmp(&a.1));
    for j in 0..k {
        result.push(folded[j as usize].0);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let test_input_nums = vec![1, 2, 2, 3, 3, 3];
        let test_input_k = 2;
        let expected_output = vec![3, 2];

        let result = top_k_freq_ele(test_input_nums, test_input_k);

        assert_eq!(result, expected_output);
    }
}
