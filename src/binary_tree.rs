use rand::Rng;
use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    height: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    pub fn height(&self) -> i8 {
        match self.0 {
            Some(ref t) => t.height,
            None => 0,
        }
    }

    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.height = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        match self.0 {
            Some(ref mut node) => {
                if data < node.data {
                    node.left.add_sorted(data);
                } else {
                    node.right.add_sorted(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    height: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }))
            }
        }
        self.set_height();
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, depth: u32) {
        if let Some(ref node) = self.0 {
            node.left.print_lfirst(depth + 1);
            let mut branch = String::new();
            for _ in 0..node.height {
                branch.push('-');
            }
            println!("{}:{}{:?}", node.height, branch, node.data);
            node.right.print_lfirst(depth + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let base_vec: Vec<u64> = (0..10).map(|_| rng.gen_range(0, 100)).collect();
        let mut bt = BinTree::new();
        println!("{:?}", base_vec);

        for i in base_vec.iter() {
            bt.add_sorted(i);
        }

        bt.print_lfirst(0);
        panic!("!");
    }
}
