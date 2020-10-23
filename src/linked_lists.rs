#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn value(self) -> T {
        self.0.unwrap().0
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linked_list_push_front() {
        let mut ll: LinkedList<u32> = LinkedList::new();

        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);

        println!("valu: {:?}", ll.value());

        panic!("");


    }
    fn linked_list_push_back() {}
    fn linked_list_sort() {}
    fn linked_list_add_sorted() {}
}
