use std::ops::Neg;

fn solve_three_sums(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort_by(|a, b| a.cmp(b));
    /*sort the array first
     * the formula is a+b+c = 0,
     * simplied -c = a + b
     */
    let len = sorted_nums.len();
    let mut result = vec![];
    let mut last_c = sorted_nums[0];
    for (index_c, c) in sorted_nums.iter().enumerate() {
        if index_c != 0 && &last_c == c {
            continue;
        } else {
            last_c = c.clone();
        }
        let target = c*-1;
        let mut l_ptr = index_c + 1;
        let mut r_ptr = len - 1;
        while l_ptr < r_ptr {
            println!("{:?},  {:?} , {:?}",  c, sorted_nums[l_ptr], sorted_nums[r_ptr]);
            if target == sorted_nums[l_ptr] + sorted_nums[r_ptr] {
                result.push(vec![ c.clone(), sorted_nums[l_ptr], sorted_nums[r_ptr]]);
                l_ptr += 1;
                r_ptr -= 1;
                //avoid duplicates
                while l_ptr < len - 1 && r_ptr > 0 && sorted_nums[l_ptr] == sorted_nums[l_ptr - 1] && sorted_nums[r_ptr] == sorted_nums[r_ptr + 1] {
                    l_ptr += 1;
                    r_ptr -= 1;
                }
            } else if target > sorted_nums[l_ptr] + sorted_nums[r_ptr] {
                l_ptr += 1;
            } else {
                r_ptr -= 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let expected_output = vec![[-1, -1, 2], [-1, 0, 1]];

        let result = solve_three_sums(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn testcase_2() {
        let input = vec![0, 1, 1];
        let expected_output: Vec<Vec<i32>> = vec![];

        let result = solve_three_sums(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn testcase_3() {
        let input = vec![0, 0, 0];
        let expected_output = vec![[0, 0, 0]];

        let result = solve_three_sums(input);

        assert_eq!(result, expected_output);
    }

    #[test]
    fn testcase_4() {
        let input = vec![0, 0, 0, 0];
        let expected_output = vec![[0, 0, 0]];

        let result = solve_three_sums(input);

        assert_eq!(result, expected_output);
    }
    
    #[test]
    fn testcase_5() {
        let input = vec![1,-1,-1,0];
        let expected_output = vec![[-1,0,1]];

        let result = solve_three_sums(input);

        assert_eq!(result, expected_output);
    }
}
