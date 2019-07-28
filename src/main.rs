use std::process;
use std::cmp;

struct  World {
    size: usize,
    elements: Vec<bool>,
}

impl World {
    fn new(size: usize) -> World {
        let elements: Vec<bool> = vec![false; size*size];
        World {size, elements}
    }

    fn get(&self, row: usize, col: usize) -> &bool {
        let idx = self.size * row + col;

        match self.elements.get(idx) {
            Some(result) => return result,
            None => {
                eprintln!("Error: Invalid access attempt in World::get({},{})", row, col);
                process::exit(1);
            }
        }
    }

    fn set(&mut self, row: usize, col: usize, value: bool) {

        if cmp::max(row, col) >= self.size {
            eprintln!("Error: Invalid access attempt in World::get({},{})", row, col);
            process::exit(1);
        }

        let idx = self.size * row + col;

        self.elements[idx] = value;
    }
}

fn main() {
    let mut world = World::new(3);

    println!("Size: {}", world.size);
    println!("Elements: {:?}", world.elements);
    println!("(0,0) element: {}", world.get(0,0));

    world.set(0,1,true);

    println!("(0,1) element: {}", world.get(0,1));
}
