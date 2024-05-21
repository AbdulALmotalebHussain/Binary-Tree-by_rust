struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    fn new() -> BinaryTree {
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));

        match self.root {
            None => self.root = Some(new_node),

            Some(ref mut node) => {
                let mut current = node;

                loop {
                    if value < current.value {
                        if let Some(ref mut left) = current.left {
                            current = left;
                        } else {
                            current.left = Some(new_node);

                            break;
                        }
                    } else {
                        if let Some(ref mut right) = current.right {
                            current = right;
                        } else {
                            current.right = Some(new_node);

                            break;
                        }
                    }
                }
            }
        }
    }

    fn in_order_traversal(&self) {
        fn traverse(node: &Option<Box<Node>>) {
            if let Some(ref n) = node {
                traverse(&n.left);

                print!("{} ", n.value);

                traverse(&n.right);
            }
        }

        traverse(&self.root);

        println!();
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(5);

    tree.insert(3);

    tree.insert(7);

    tree.insert(1);

    tree.insert(4);

    tree.insert(0);

    tree.insert(6);

    tree.in_order_traversal(); // 1 3 4 5 7
}
