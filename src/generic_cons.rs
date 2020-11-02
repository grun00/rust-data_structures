use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, data: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head() {
        let ll = List {
            head: Some(Rc::new(Node {
                data: 1,
                next: None,
            })),
        };

        assert_eq!(ll.head(), Some(&1));
    }
    #[test]
    fn append() {
        let mut ll: List<u32> = List::new();

        ll = ll.append(1);
        assert_eq!(ll.head(), Some(&1));
        ll = ll.append(2);
        assert_eq!(ll.head(), Some(&2));
        ll = ll.append(3);
        assert_eq!(ll.head(), Some(&3));
    }
    #[test]
    fn tail() {
        let mut ll: List<u32> = List::new();

        ll = ll.append(1);
        ll = ll.append(2);
        ll = ll.append(3);

        ll = ll.tail();
        assert_eq!(ll.head(), Some(&2));
        ll = ll.tail();
        assert_eq!(ll.head(), Some(&1));
        ll = ll.tail();
        assert_eq!(ll.head(), None);
    }
    #[test]
    fn iteration() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
