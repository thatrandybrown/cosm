mod world;

use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::{Rc, Weak};

use rand::prelude::*;
use std::thread;
use std::time::Duration;

use world::World;

fn main() {
    let mut world = World::new();
    // let mut rng = rand::rng();

    while world.cells().iter().any(|&cell| cell) {
        world = world.next();

        let cells = world.cells();
        let nh_raw = cells.iter().take(5).cloned();
        let sh_raw = cells.iter().skip(5).cloned();

        let row1 = nh_raw.clone().map(|s| if s { '▲' } else { '△' });

        let row3 = sh_raw.clone().map(|s| if s { '▼' } else { '▽' });

        let row2 = sh_raw
            .clone()
            .map(|s| if s { '▲' } else { '△' })
            .zip(nh_raw.map(|s| if s { '▼' } else { '▽' }))
            .flat_map(|(a, b)| [a, b]);

        println!("{}", row1.map(|c| format!(" {}", c)).collect::<String>());
        println!("{}", row2.collect::<String>());
        print!("{}", row3.map(|c| format!("{} ", c)).collect::<String>());

        print!("\x1B[2A\r");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}
