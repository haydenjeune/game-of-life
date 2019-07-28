struct  World {
    size: usize,
    elements: Vec<bool>,
}

impl World {
    fn new(size: usize) -> World {
        let elements: Vec<bool> = vec![false; size*size];
        World {size, elements}
    }

    fn get(&self, row: &usize, col: &usize) -> Option<&bool> {
        let idx = self.size * row + col;

        self.elements.get(idx)
    }
}

fn main() {
    let world = World::new(3);

    println!("Size: {}", world.size);
    println!("Elements: {:?}", world.elements)

}
