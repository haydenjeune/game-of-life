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

    fn get(&self, row: usize, col: usize) -> Result<bool, &'static str> {

        if cmp::max(row, col) >= self.size {
            return Err("Error: Invalid access attempt in World::set()");
        }

        let idx = self.size * row + col;

        let value = self.elements[idx];
        Ok(value)
    }

    fn set(&mut self, row: usize, col: usize, value: bool) -> Result<(), &'static str> {

        if cmp::max(row, col) >= self.size {
            return Err("Error: Invalid access attempt in World::set()");
        }

        let idx = self.size * row + col;

        self.elements[idx] = value;
        Ok(())
    }
}

fn main() {
    let mut world = World::new(3);

    println!("Size: {}", world.size);
    println!("Elements: {:?}", world.elements);

    println!("(0,0) element: {:?}", world.get(0,0).unwrap());

    world.set(0,1,true).unwrap();

    println!("(0,1) element: {}", world.get(0,1).unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let world = World::new(3);

        assert_eq!(
            world.size,
            3
        );

        assert_eq!(
            world.elements,
            vec![false; 9]
        );
    }

    #[test]
    fn get_valid() {
        let world = World::new(3);
        
        assert_eq!(
            world.get(1,2),
            Ok(false)
        );
    }

    #[test]
    fn get_invalid() {
        let world = World::new(3);

        assert!(world.get(1,3).is_err())
    }

    #[test]
    fn set_valid() {
        let mut world = World::new(3);
        
        assert_eq!(
            world.set(1, 2, true),
            Ok(())
        );

        assert_eq!(
            world.get(1, 2),
            Ok(true)
        );
    }

    #[test]
    fn set_invalid() {
        let mut world = World::new(3);
        
        assert!(world.set(1, 3, true).is_err())
    }
}