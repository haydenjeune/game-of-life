use std::process;

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
}

fn main() {
    let world = World::new(3);

    println!("Size: {}", world.size);
    println!("Elements: {:?}", world.elements);
    println!("First element: {}", world.get(0,0));

}
