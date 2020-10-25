use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T: PartialOrd>(Option<(T, Box<Node<T>>)>);

impl<T: std::fmt::Debug + PartialOrd> Node<T> {
    pub fn new() -> Self {
        Node(None)
    }

    pub fn new_head(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(Node(t))));
    }

    pub fn append(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.append(data),
            None => self.new_head(data),
        }
    }

    pub fn add_in_order(&mut self, data: T) {
        match self.0 {
            Some((ref value, ref mut child)) => {
                if  data <= *value {
                    self.new_head(data);
                } else {
                    child.add_in_order(data);
                }
            },
            None => self.new_head(data),
        }
    }

    pub fn make_array(self) -> Vec<T> {
        let mut array: Vec<T> = Vec::new();
        let mut nav = self;

        loop {
            match nav.0 {
                Some((value,child)) => {
                    array.push(value);
                    nav = *child;
                },
                None => break
            }
        }

        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_append() {
        let mut node: Node<u32> = Node::new();

        node.append(1);
        node.append(2);

        assert_eq!(node.0.unwrap().0, 1);
    }
    #[test]
    fn linked_list_add_head() {
        let mut node: Node<u32> = Node::new();

        node.new_head(1);
        node.new_head(2);

        assert_eq!(node.0.unwrap().0, 2);
    }
    #[test]
    fn linked_list_add_in_order() {
        let mut node: Node<u32> = Node::new();

        for i in 0..=5 {
            node.append(i * 2);
        }

        for i in 1..=5 {
            node.add_in_order(i * 2 - 1);
        }

        let vec = node.make_array();

        assert_eq!(vec, [0,1,2,3,4,5,6,7,8,9,10]);
    }
}
