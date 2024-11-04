// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_reader = l1.clone();
        let mut l2_reader = l2.clone();
        let mut ans = None;
        let mut tail = &mut ans;
        let mut carry = 0;

        while l1_reader.is_some() || l2_reader.is_some() || carry != 0 {
            let l1_val = match l1_reader {
                Some(node) => {
                    l1_reader = node.next;
                    node.val
                }
                None => 0,
            };
            let l2_val = match l2_reader {
                Some(node) => {
                    l2_reader = node.next;
                    node.val
                }
                None => 0
            };
            
            let sum = l1_val + l2_val + carry;
            carry = sum / 10;

            *tail = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut tail.as_mut().unwrap().next;
        };
        ans
    }
}