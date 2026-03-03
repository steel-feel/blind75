// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Make list like [0, n-1, 1, n-2, 2, n-3, ...]
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return;
    }

    // 1. Find the length to locate the middle
    let mut len = 0;
    {
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }
    }

    // 2. Split the list in-place
    let mut curr_mut = head.as_mut();
    for _ in 0..(len - 1) / 2 {
        curr_mut = curr_mut.unwrap().next.as_mut();
    }
    let second_half_head = curr_mut.unwrap().next.take();

    // 3. Reverse the second half
    let mut prev = None;
    let mut curr = second_half_head;
    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }
    let mut second_half = prev;

    // 4. Combine the lists alternatively
    let mut first_half = head.as_mut();
    while let Some(first_node) = first_half {
        if let Some(mut second_node) = second_half {
            let first_next = first_node.next.take();
            let second_next = second_node.next.take();

            // Link: First -> Second -> Original First Next
            second_node.next = first_next;
            first_node.next = Some(second_node);

            // Move cursor: jump past the inserted second node
            first_half = first_node.next.as_mut().unwrap().next.as_mut();
            second_half = second_next;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_linkedlist_from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head_node = None;
        let mut curr_node = &mut head_node;
        for val in nums {
            let new_node = Box::new(ListNode::new(val));
            *curr_node = Some(new_node);
            curr_node = &mut curr_node.as_mut().unwrap().next;
        }
        head_node
    }

    #[test]
    fn testcase_1() {
        let input_vec = vec![2, 4, 6, 8];
        let mut head_node = create_linkedlist_from_vec(input_vec);

        let expected_vec = vec![2, 8, 4, 6];
        let expected_output_node = create_linkedlist_from_vec(expected_vec);

        reorder_list(&mut head_node);

        let mut current_result = &head_node;
        let mut current_expected = &expected_output_node;
        while let (Some(result_node), Some(expected_node)) = (current_result, current_expected) {
            assert_eq!(result_node.val, expected_node.val);
            current_result = &result_node.next;
            current_expected = &expected_node.next;
        }
        assert!(current_result.is_none());
        assert!(current_expected.is_none());
    }

    #[test]
    fn testcase_2() {
        let input_vec = vec![1, 2, 3, 4, 5];
        let mut head_node = create_linkedlist_from_vec(input_vec);

        let expected_vec = vec![1, 5, 2, 4, 3];
        let expected_output_node = create_linkedlist_from_vec(expected_vec);

        reorder_list(&mut head_node);

        let mut current_result = &head_node;
        let mut current_expected = &expected_output_node;
        while let (Some(result_node), Some(expected_node)) = (current_result, current_expected) {
            assert_eq!(result_node.val, expected_node.val);
            current_result = &result_node.next;
            current_expected = &expected_node.next;
        }
        assert!(current_result.is_none());
        assert!(current_expected.is_none());
    }
}
