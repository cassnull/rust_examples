use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    pub length: u64,
}

impl<T> DoublyLinkedList<T> {
    pub fn new(value: T) -> Self {
        let new_head = Node::new(value);

        Self {
            head: Some(new_head.clone()),
            tail: Some(new_head),
            length: 1,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

pub struct ListIterator<T> {
    current: Link<T>,
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl<T: Clone> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

fn main() {}
