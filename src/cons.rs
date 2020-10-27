use std::mem;

#[allow(dead_code)]
#[derive(Debug)]
pub struct List {
    head: Link,
}
#[allow(dead_code)]
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}
#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    data: u32,
    next: Link,
}

#[allow(dead_code)]
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, data: u32) {
        let new_node = Box::new(Node {
            data,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<u32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn nth_element(&self, nth: u32) -> Option<u32> {
        let mut nav = &self.head;

        for _ in 0..nth {
            match nav {
                Link::Empty => {}
                Link::More(node) => {
                    nav = &node.next;

                }
            }
        }

        match nav {
            Link::Empty => None,
            Link::More(node) => Some(node.data)
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn push() {
        let mut cons = List::new();

        cons.push(10);
        match &cons.head {
            Link::Empty => { panic!(""); },
            Link::More(node) => { assert_eq!(10, node.data) }
        }
    }

    #[test]
    fn pop() {
        let mut cons = List::new();
        cons.push(10);
        cons.push(20);

        assert_eq!(cons.pop(), Some(20));
        assert_eq!(cons.pop(), Some(10));
    }

    #[test]
    fn nth_element() {
        let mut cons = List::new();

        for i in (0..10).rev() {
            cons.push(i);
        }

        for i in 0..10 {
            assert_eq!(cons.nth_element(i), Some(i));
        }
    }
}
