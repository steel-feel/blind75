pub fn prod_array_exc_self(nums: Vec<u32>) -> Vec<u32> {
    let mut prefix_prod = 1;
    let mut prefix_arr = vec![];
   
    let mut suffix_prod = 1;
    let mut suffix_arr = vec![];
    
    let nums_len = nums.len();
    for i in 0..nums_len {
        prefix_prod *= nums[i];
        prefix_arr.push(prefix_prod);
        
        suffix_prod *= nums[nums_len - 1 - i];
        suffix_arr.push(suffix_prod);
        /*
         * 1,2,3,4
         * 
         * suffix : [4, 12, 24, 24] , product till from len - 1  to len - 1 - i = suffix[i], 
         * prefix : [1, 2, 6, 24]
         */
    }
    let mut result =  vec![];
    for i in 0..nums_len {
        //calc prefix arr index 
        let prefix_prod = if i > 0 {
            prefix_arr[i-1]
        } else {
            1
        };
        
        let suffix_prod = if i != nums_len - 1 {
            suffix_arr[nums_len - 2 - i]
        } else {
            1
        };
        
        // calc suffix arr index
        result.push(prefix_prod*suffix_prod);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let test_input = vec![1,2,4,6];
        let expected_output = vec![48,24,12,8];
        
        let result = prod_array_exc_self(test_input);
        
        assert_eq!(result, expected_output);
    }
    

}