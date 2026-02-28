fn solve_largest_area_bar (heights: Vec<u32>) -> u32 {
    let mut l_ptr = 0;
    let mut r_ptr = heights.len() -1;
    let mut max_area = 0;
    while r_ptr > l_ptr {
        let current_max_area = heights[l_ptr].min(heights[r_ptr]) * (r_ptr - l_ptr) as u32;
        if max_area < current_max_area {
            max_area = current_max_area;
        }
        
        if heights[l_ptr].gt(&heights[r_ptr]) {
            r_ptr -=1;
        } else {
            l_ptr += 1;
        }
        
    }
    
    max_area
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let test_input = vec![1,7,2,5,4,7,3,6];
        let expected_output = 36;
        
        let result = solve_largest_area_bar(test_input);
        
        assert_eq!(result,expected_output);
    }
    
    #[test]
    fn testcase_2() {
        let test_input = vec![2,2,2];
        let expected_output = 4;
        
        let result = solve_largest_area_bar(test_input);
        
        assert_eq!(result,expected_output);
    }

}