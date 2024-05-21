# BinaryTree-byrust

This repository contains a simple implementation of a binary tree in Rust. The code defines a `Node` struct and a `BinaryTree` struct, with methods for creating new nodes, inserting values into the tree, and performing an in-order traversal.

## Code Overview

### Node Struct

The `Node` struct represents a node in the binary tree. Each node contains a value and optional references to its left and right children.

```rust
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
```

### BinaryTree Struct

The `BinaryTree` struct represents the binary tree itself. It contains a reference to the root node and methods for creating a new tree, inserting values, and performing an in-order traversal.

```rust
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
```

### Main Function

The `main` function demonstrates the usage of the `BinaryTree` struct. It creates a new tree, inserts several values, and performs an in-order traversal to print the values in ascending order.

```rust
fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(0);
    tree.insert(6);

    tree.in_order_traversal(); // 0 1 3 4 5 6 7
}
```

## Usage

To run this code, you need to have Rust installed. If Rust is not installed, you can download it from [the official Rust website](https://www.rust-lang.org/).

1. Clone this repository.
2. Navigate to the directory containing the code.
3. Compile and run the code using Cargo:

```sh
cargo run
```

## Features

- Create a new binary tree.
- Insert values into the binary tree.
- Perform an in-order traversal to print values in ascending order.

## Node Tree

Here is the node tree structure after inserting the values 5, 3, 7, 1, 4, 0, 6:

```
        5
       / \
      3   7
     / \   \
    1   4   6
   /
  0
```

In-order traversal of this tree will result in the following output:

```
0 1 3 4 5 6 7
```

This repository serves as a basic example of how to implement and manipulate a binary tree in Rust.
