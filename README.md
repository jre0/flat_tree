# Organization Tree
This is a rehash of the organization tree structure. The Organization structure follows the JS object pattern. Nodes do not own nodes in this case. A node is simply a string that is used to access another array of strings. The ```flat``` method uses the same logic as the ```Node``` struct. This rehash also includes an import statement to grab the data from JS Obj / JSON serial data. 

## Install 
Please follow these instructions to install Rust: https://www.rust-lang.org/learn/get-started

## Test
```cd``` into flat_tree root and run ```cargo test -- --nocapture``` to see results. Cargo package manager will build and run tests. The result should be:
```names: ["Jane Mayer", "Baraka Tumuti", "Abida Begum", "Dave Bunt", "James Ray", "Sarah Lee", "David Gibbly", "Kelsey Hamming", "David Heinsburg"]```
