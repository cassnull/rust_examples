use std::cell::RefCell;
use std::rc::Rc;

type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(PartialEq, Clone, Debug)]
pub struct Node<T> {
    value: T,
    next: SingleLink<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(PartialEq, Debug)]
pub struct LinkedList<T> {
    head: SingleLink<T>,
    tail: SingleLink<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new(value: T) -> Self {
        let new_head = Node::new(value);

        LinkedList {
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

    pub fn push_front(&mut self, value: T) {
        let new_head = Node::new(value);
        match self.head.take() {
            Some(old_head) => new_head.borrow_mut().next = Some(old_head),
            None => self.tail = Some(new_head.clone()),
        }
        self.head = Some(new_head);
        self.length += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_head = Node::new(value);
        match self.tail.take() {
            Some(old_head) => old_head.borrow_mut().next = Some(new_head.clone()),
            None => self.head = Some(new_head.clone()),
        };
        self.tail = Some(new_head);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new("node_1".to_string());
        assert_eq!(
            node,
            Rc::new(RefCell::new(Node {
                value: "node_1".to_string(),
                next: None
            }))
        )
    }

    #[test]
    fn test_new_empty_list() {
        let list: LinkedList<String> = LinkedList::new_empty();

        assert_eq!(
            list,
            LinkedList {
                head: None,
                tail: None,
                length: 0
            }
        )
    }

    #[test]
    fn test_new_list() {
        let list = LinkedList::new("node_1".to_string());

        assert_eq!(
            list,
            LinkedList {
                head: Some(Node::new("node_1".to_string())),
                tail: Some(Node::new("node_1".to_string())),
                length: 1
            }
        )
    }

    #[test]
    fn test_append_start() {
        let s = "tail".to_string();
        let c = "head".to_string();

        let tail = Node::new(s.clone());

        let head = Node {
            value: c.clone(),
            next: Some(Rc::clone(&tail)),
        };

        let list = LinkedList {
            head: Some(Rc::new(RefCell::new(head))),
            tail: Some(tail),
            length: 2,
        };

        let mut l_list = LinkedList::new_empty();
        l_list.push_front(s);
        l_list.push_front(c);
        assert_eq!(l_list, list);
    }
    #[test]
    fn test_append_end() {
        let s = "head".to_string();
        let c = "tail".to_string();

        let tail = Node::new(c.clone());

        let head = Node {
            value: s.clone(),
            next: Some(Rc::clone(&tail)),
        };

        let list = LinkedList {
            head: Some(Rc::new(RefCell::new(head))),
            tail: Some(tail),
            length: 2,
        };

        let mut l_list = LinkedList::new(s);
        l_list.push_back(c);
        assert_eq!(l_list, list);
    }

    #[test]
    fn test_append_end1() {
        let s = "head".to_string();
        let c = "tail".to_string();
        let m = "middle".to_string();

        let tail = Node::new(c.clone());

        let mut l_list = LinkedList::new(s);
        l_list.push_back(m);
        l_list.push_back(c);
        assert_eq!(l_list.length, 3);
        assert_eq!(l_list.tail, Some(tail));
    }

    #[test]
    fn test_pop_start() {
        let s = "head".to_string();
        let c = "tail".to_string();
        let m = "middle".to_string();

        let tail = Node::new(c.clone());

        let middle = Node {
            value: m.clone(),
            next: Some(Rc::clone(&tail)),
        };

        let list = LinkedList {
            head: Some(Rc::new(RefCell::new(middle))),
            tail: Some(tail),
            length: 2,
        };

        let mut l_list = LinkedList::new(s);
        l_list.push_back(m);
        l_list.push_back(c);
        l_list.pop_front();
        assert_eq!(l_list, list);
    }
}
