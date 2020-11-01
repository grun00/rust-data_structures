use std::rc::Rc;

#[allow(dead_code)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
