// use std::cell::RefCell;
// use std::rc::Rc;
//
// pub struct List<T> {
//     head: Link<T>,
//     tail: Link<T>,
// }
//
// type Link<T> = Option<Rc<RefCell<Node<T>>>>;
//
// struct Node<T> {
//     data: T,
//     next: Link<T>,
//     prev: Link<T>,
// }
//
// impl<T> Node<T> {
//     fn new(data: T) -> Rc<RefCell<Self>> {
//         Rc::new(RefCell::new(Node {
//             data,
//             prev: None,
//             next: None,
//         }))
//     }
// }
//
// impl<T> List<T> {
//     pub fn new() -> Self {
//         List {
//             head: None,
//             tail: None,
//         }
//     }
//
//     pub fn push_front(&mut self, data: T) {
//         let new_head = Node::new(data);
//         match self.head.take() {
//             Some(old_head) => {
//                 old_head.prev = Some(new_head.clone());
//                 new_head.next = Some(old_head);
//                 self.head = Some(new_head);
//             }
//             None => {
//                 self.tail = Some(new_head.clone());
//                 self.head = Some(new_head);
//             }
//         }
//     }
// }
