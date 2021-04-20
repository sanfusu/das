use core::ptr::NonNull;

use alloc::boxed::Box;

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    ele: T,
}

impl<T> Node<T> {
    pub fn new(ele: T) -> Box<Self> {
        Box::new(Self { next: None, ele })
    }

    /// 将新节点插入到当前节点之后。
    pub fn insert_after(&mut self, mut node: Box<Node<T>>) {
        node.next = self.next;
        self.next = Some(Box::leak(node).into());
    }

    /// 移除下一个节点，并返回。
    pub fn remove_next(&mut self) -> Option<Box<Self>> {
        let node = match self.next {
            Some(mut next) => unsafe {
                self.next = (*next.as_ptr()).next;
                Some(Box::from_raw(next.as_mut()))
            },
            None => None,
        };
        node
    }
}

pub struct SingleLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn append(&mut self, ele: T) {
        let new_node = Some(Box::leak(Node::new(ele)).into());
        match self.tail {
            Some(mut node) => unsafe {
                node.as_mut().next = new_node;
                self.tail = node.as_ref().next;
            },
            None => {
                self.head = new_node;
            }
        }
        self.len += 1;
    }
}