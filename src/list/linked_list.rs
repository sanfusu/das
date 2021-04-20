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
    len: usize,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }
    /// 插入到最前面
    pub fn push(&mut self, ele: T) {
        let mut new_node: NonNull<Node<T>> = Box::leak(Node::new(ele)).into();
        unsafe {
            new_node.as_mut().next = self.head;
        }
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut first) = self.head {
            let node = unsafe {
                self.head = first.as_ref().next;
                Box::from_raw(first.as_mut())
            };
            self.len -= 1;
            return Some(node.ele);
        }
        None
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn sll_test() {
        let mut list = super::SingleLinkedList::<u32>::new();
        for i in 0..10 {
            list.push(i);
        }
        println!("list length: {}", list.len);

        while let Some(ele) = list.pop() {
            print!("{} ", ele);
        }

        println!("list length: {}", list.len);
    }
}
