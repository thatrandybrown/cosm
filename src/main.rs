use std::rc::{Weak};
use std::cell::RefCell;

struct Cell {
    state: bool,
    neighbors: Option<Vec<Weak<RefCell<Cell>>>>
}

fn main() {
    let mut world = vec![];
    for _ in 0..10 {
        let cell = Cell {
            state: false,
            neighbors: None,
        };
        world.push(Rc::new(RefCell::new(cell)));
    }
    println!("Hello, world!");
}
