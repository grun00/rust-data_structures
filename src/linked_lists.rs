#[allow(dead_code)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.data
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn push() {
        let mut ll = List::new();

        ll.push(10);
        match &ll.head {
            Link::None => { panic!(""); },
            Link::Some(node) => { assert_eq!(10, node.data) }
        }
    }
    #[test]
    fn pop() {
        let mut ll = List::new();
        ll.push("123");
        ll.push("456");

        assert_eq!(ll.pop(), Some("456"));
        assert_eq!(ll.pop(), Some("123"));
    }
    #[test]
    fn peek() {
        let mut ll: List<u32> = List::new();

        assert_eq!(ll.peek(), None);

        ll.push(1);
        assert_eq!(ll.peek(), Some(&1));
    }
    #[test]
    fn peek_mut() {
        let mut ll: List<u32> = List::new();

        assert_eq!(ll.peek_mut(), None);

        ll.push(1);
        assert_eq!(ll.peek_mut(), Some(&mut 1));

        ll.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(ll.peek_mut(), Some(&mut 42));
    }
    #[test]
    fn sort() {}
}
