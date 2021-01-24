/*
You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero, except the number 0 itself.
*/

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut carry = 0;

        let mut result = ListNode::new(0);
        let mut current_node = &mut result;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;
            }

            carry = sum / 10;
            current_node.next = Some(Box::new(ListNode::new(sum % 10)));
            current_node = current_node.next.as_mut().unwrap();
        }

        return result.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(
            ListNode {
                val: 2,
                next: Some(Box::new(
                    ListNode {
                        val: 4,
                        next: Some(Box::new(
                            ListNode { val: 3, next: None })),
                    }
                )),
            }
        ));
        let l2 = Some(Box::new(
            ListNode {
                val: 5,
                next: Some(Box::new(
                    ListNode {
                        val: 6,
                        next: Some(Box::new(
                            ListNode { val: 4, next: None })),
                    }
                )),
            }
        ));
        let expected = Some(Box::new(ListNode { val: 7, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode::new(8))) })) }));
// 7 0 8
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}