fn main() {
    let head = build_list();
    print_list(&head);
    let newHead = Solution::reverse_list(head);
    print_list(&newHead);
}

/**
 * 206 反转单链表
 */

struct Solution {}

impl Solution {
    /**
     * 迭代写法
     */
    // pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut pre = None;
    //     let mut cur = head;
    //     while let Some(mut node) = cur {
    //         let next = node.next.take();
    //         node.next = pre;
    //         pre = Some(node);
    //         cur = next;
    //     }
    //     return pre;
    // }

    /**
     * 递归写法，内部声明了一个局部函数，感觉像是用递归实现了迭代的写法hhh
     * rust因为所有权的原因，真正的递归写法很难实现，所以只能这么写
     */
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn helper(
            head: Option<Box<ListNode>>,
            tail: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match head {
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = tail;
                    helper(next, Some(node))
                }
                None => tail,
            }
        }
        helper(head, None)
    }
}

fn build_list() -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut p = head.as_mut();
    for i in 2..6 {
        let next = Some(Box::new(ListNode::new(i)));
        if let Some(node) = p {
            node.next = next;
            p = node.next.as_mut();
        }
    }
    return head;
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut p = head;
    while let Some(cur) = p {
        print!("{} ", cur.val);
        p = &cur.next;
    }
    println!()
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
