use std::rc::{Weak};
use std::cell::RefCell;

struct Cell {
    state: bool,
    neighbors: Vec<Weak<RefCell<Cell>>>
}

fn main() {
    println!("Hello, world!");
}
