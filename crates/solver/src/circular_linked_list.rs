#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<NodeRef<T>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> where T: Copy {
    fn get_value(&self) -> T {
        self.value
    }
}

#[derive(Debug)]
struct CircularLinkedList<T> {
    head: Link<T>,
}

impl<T> CircularLinkedList<T>
where
    T: std::fmt::Debug,
{
    fn new() -> Self {
        CircularLinkedList { head: None }
    }

    fn append(&mut self, value: T) -> NodeRef<T> {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
        }));

        match &self.head {
            Some(head) => {
                let mut tail = head.clone();
                while tail.borrow().next.as_ref().is_some_and(|n| !Rc::ptr_eq(n, head)) {
                    let new_tail = tail.borrow().next.as_ref().unwrap().clone();
                    tail = new_tail;
                }
                tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().next = Some(head.clone());
            }
            None => {
                new_node.borrow_mut().next = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
        }
        new_node
    }

    fn get_head(&self) -> NodeRef<T> {
        self.head.as_ref().unwrap().clone()
    }

    fn insert_after_node(&mut self, node: NodeRef<T>, value: T) -> NodeRef<T> {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
        }));
        
        let mut tail = node.clone();
        while tail.borrow().next.as_ref().is_some_and(|n| !Rc::ptr_eq(n, &node)) {
            let new_tail = tail.borrow().next.as_ref().unwrap().clone();
            tail = new_tail;
        }
        tail.borrow_mut().next = Some(new_node.clone());
        new_node.borrow_mut().next = Some(node.clone());

        new_node
    }

    fn display(&self, count: usize) {
        if self.head.is_none() {
            println!("List is empty");
            return;
        }

        let mut current = self.head.clone();
        for _ in 0..count {
            if let Some(node) = current {
                print!("{:?} -> ", node.borrow().value);
                current = node.borrow().next.clone();
            }
        }
        println!("...");
    }
}

#[cfg(test)]
mod circular_linked_list_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut list = CircularLinkedList::new();
    
        list.append(1);
        list.append(2);
        list.append(3);
    
        list.display(10);

        assert_eq!(0, 1);
    }
}