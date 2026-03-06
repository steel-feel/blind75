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

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    //find length of node
    let mut len = 0;
    let mut curr_node = head.as_ref();
    loop {
        match curr_node {
            Some(node) => {
                curr_node = node.next.as_ref();
                len += 1;
            }
            None => {
                break;
            }
        }
    }

    if n > len {
        return head;
    }

    let target_index = len - n; // 0-based index from head
    if target_index == 0 {
        return head.and_then(|node| node.next);
    }

    let mut curr = head.as_mut();
    for _ in 0..(target_index - 1) {
        curr = curr.and_then(|node| node.next.as_mut());
    }

    if let Some(prev) = curr {
        let next_next = prev.next.as_mut().and_then(|node| node.next.take());
        prev.next = next_next;
    }

    return head;
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

    fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut curr = head;
        while let Some(node) = curr {
            out.push(node.val);
            curr = node.next;
        }
        out
    }

    #[test]
    fn testcase_1() {
        let input_vec = vec![1, 2, 3, 4];
        let head_node = create_linkedlist_from_vec(input_vec);

        let result = remove_nth_from_end(head_node, 2);
        assert_eq!(linked_list_to_vec(result), vec![1, 2, 4]);
    }

    #[test]
    fn testcase_remove_head() {
        let head_node = create_linkedlist_from_vec(vec![10, 20]);
        let result = remove_nth_from_end(head_node, 2);
        assert_eq!(linked_list_to_vec(result), vec![20]);
    }

    #[test]
    fn testcase_single_node() {
        let head_node = create_linkedlist_from_vec(vec![7]);
        let result = remove_nth_from_end(head_node, 1);
        assert_eq!(linked_list_to_vec(result), Vec::<i32>::new());
    }
}
