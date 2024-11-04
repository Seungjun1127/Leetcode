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
    pub fn count_node(head: Option<Box<ListNode>>) -> u32 {
		let mut current = head;
		let mut cnt = 0;

		while let Some(node) = current {
			current = node.next;
			cnt += 1;
		}

		cnt
	}
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.clone();
        let len = Self::count_node(head.clone());
        let mut cnt = 0;

        while let Some(node) = current {
            cnt += 1;
            if cnt > len / 2 {
                return Some(node);
            }
            current = node.next;
        }
        None
    }
}