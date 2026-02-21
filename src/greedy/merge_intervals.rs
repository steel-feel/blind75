
/*
 * Problem : 
 * Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, 
 * and return an array of the non-overlapping intervals that cover all the intervals in the input.
 * link https://leetcode.com/problems/merge-intervals
 * Example 1:
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
 * 
 */
 
 /*
  * Solution:
  * Step 1 sort the array
  * 
  * The loop over the array
  * there could be 3 cases in total
  * 1. simple if the end prev interval is smaller than start of current interval, create a new entry
  * 2. else there could be 2 more possibilites
  *     a. if prev end is greater than current end, nothing to-do since this interval completely collapse
  *     b. if prev end is smaller than current end, replace prev end with current end
  */

fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted = intervals.clone();
    // Sort the array
    sorted.sort_by(|a, b| a[0].cmp(&b[0]));
    
    println!("sorted {:?}", sorted);

    
    
    let mut fin: Vec<Vec<i32>> = vec![sorted[0].clone()];
    // let mut start = fin[0][0];
    // let mut end = fin[0][1];
    for interval in &sorted[1..] {
        let last_index = fin.len() - 1;
        println!("loop start fin {:?} , fin.len() {} ", fin ,fin.len() );
        if fin[last_index][1] < interval[0] {
            fin.push(interval.clone());
            continue;
        } else {
            if interval[1] < fin[last_index][1] {
                continue;
            }
            fin[last_index][1] = interval[1];
              println!("fin {:?}", fin);
        }
    }

    fin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let test = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];

        let result = merge_intervals(test);
        let expected = vec![vec![1,6],vec![8,10],vec![15,18]];

        assert_eq!(result, expected);
    }
}
