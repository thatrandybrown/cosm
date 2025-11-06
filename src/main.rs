
use std::rc::{Rc, Weak};
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

    for cell_rc in &world {
        let mut cell = cell_rc.borrow_mut();
        cell.neighbors = Some(vec![Rc::downgrade(&world[0]), Rc::downgrade(&world[1])]);
    }

    println!("Hello, world!");
}
