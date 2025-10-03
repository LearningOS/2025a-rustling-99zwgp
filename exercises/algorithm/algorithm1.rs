/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
// use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut result = LinkedList::new();

        // 直接操作链表的 start 指针，避免额外引用导致的冲突
        while list_a.start.is_some() && list_b.start.is_some() {
            let a_ptr = list_a.start.unwrap();
            let b_ptr = list_b.start.unwrap();

            let a_val = unsafe { &(*a_ptr.as_ptr()).val };
            let b_val = unsafe { &(*b_ptr.as_ptr()).val };

            if a_val <= b_val {
                // 从链表a转移节点
                let node = list_a.take_first();
                result.push_node(node);
            } else {
                // 从链表b转移节点
                let node = list_b.take_first();
                result.push_node(node);
            }
        }

        // 转移链表a剩余节点
        while list_a.start.is_some() {
            let node = list_a.take_first();
            result.push_node(node);
        }

        // 转移链表b剩余节点
        while list_b.start.is_some() {
            let node = list_b.take_first();
            result.push_node(node);
        }

        result
    }

    // 辅助方法：取出链表的第一个节点
    fn take_first(&mut self) -> NonNull<Node<T>> {
        let first_ptr = self.start.unwrap();

        // 更新链表的start指针到下一个节点
        let next_node = unsafe { (*first_ptr.as_ptr()).next };
        self.start = next_node;

        // 如果链表变为空，更新end指针
        if next_node.is_none() {
            self.end = None;
        }

        self.length -= 1;

        // 确保取出的节点next为None
        unsafe {
            (*first_ptr.as_ptr()).next = None;
        }

        first_ptr
    }

    // 辅助方法：将节点添加到链表末尾
    fn push_node(&mut self, node_ptr: NonNull<Node<T>>) {
        match self.end {
            None => {
                // 链表为空时，节点既是头也是尾
                self.start = Some(node_ptr);
                self.end = Some(node_ptr);
            }
            Some(end_ptr) => {
                // 链表非空时，添加到末尾
                unsafe {
                    (*end_ptr.as_ptr()).next = Some(node_ptr);
                }
                self.end = Some(node_ptr);
            }
        }
        self.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

// 实现Drop trait以避免内存泄漏
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            current = unsafe { (*node_ptr.as_ptr()).next.take() };
            // 将原始指针转换回Box，当Box离开作用域时会自动释放内存
            unsafe {
                let _ = Box::from_raw(node_ptr.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
