use std::cmp;
use rand::random;

#[derive(Clone)]
pub struct World {
    pub size: usize,
    elements: Vec<bool>,
}

impl World {
    pub fn new(size: usize) -> World {
        // 2 is added to pad the outside of the vector with false
        let num_of_elements = (size + 2) * (size + 2);

        let elements: Vec<bool> = vec![false; num_of_elements];

        World {size, elements}
    }

    pub fn randomise(&mut self) -> Result<(), &'static str> {
        for row in 0..self.size {
            for col in 0..self.size {
                self.set(row, col, random())?;
            }
        }
        Ok(())
    }

    fn _get_vector_index(&self, row: usize, col: usize) -> usize {
        (self.size + 2) * row + col
    }

    fn _get_unmapped(&self, row: usize, col: usize) -> bool {
        let idx = self._get_vector_index(row, col);
        self.elements[idx]
    }

    pub fn get(&self, row: usize, col: usize) -> Result<bool, &'static str> {

        if cmp::max(row, col) >= self.size {
            return Err("Error: Invalid access attempt in World::get()");
        }

        // map row and col to ignore padding
        let value = self._get_unmapped(row + 1, col + 1);
        Ok(value)
    }

    fn _set_unmapped(&mut self, row: usize, col: usize, value: bool) {
        let idx = self._get_vector_index(row, col);
        self.elements[idx] = value;
    }

    pub fn set(&mut self, row: usize, col: usize, value: bool) -> Result<(), &'static str> {

        if cmp::max(row, col) >= self.size {
            return Err("Error: Invalid access attempt in World::set()");
        }

        // map row and col to ignore padding
        self._set_unmapped(row + 1, col + 1, value);
        
        Ok(())
    }

    pub fn display(&self) {
        for row in 0..self.size {
            for col in 0..self.size {
                print!("{}  ", self.get(row, col).unwrap() as u8);
            }
            print!("\n");
        }
    }

    fn get_score(&self, row: usize, col: usize) -> Result<u8, &'static str> {
        if cmp::max(row, col) >= self.size {
            return Err("Error: Invalid access attempt in World::get_score()");
        }

        let mapped_row = row + 1;
        let mapped_col = col + 1;

        let mut score: u8 = 0;
        for i in mapped_row-1..mapped_row+2 {
            for j in mapped_col-1..mapped_col+2 {
                if i != mapped_row || j != mapped_col {
                    let element = self._get_unmapped(i, j) as u8;
                    score += element;
                    //println!("get({}, {}) = {}", i, j, element);
                }
            }
        }
        Ok(score)
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let old = self.clone();

        for row in 0..old.size {
            for col in 0..old.size {
                let current = old.get(row, col);
                let next = match old.get_score(row, col) {
                    Err(e) => Err(e),
                    Ok(3) => Ok(true),
                    Ok(4) => current,
                    _ => Ok(false),
                }?;

                self.set(row, col, next)?;
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut world = World::new(3);

        assert_eq!(
            world.size,
            3
        );

        assert_eq!(
            world.elements,
            vec![false; 25]
        );
    }

    #[test]
    fn get_and_set_valid() {
        let mut world = World::new(3);
        
        assert_eq!(
            world.set(1, 2, true),
            Ok(())
        );

        assert_eq!(
            world.get(1, 2),
            Ok(true)
        );

        assert_eq!(
            world.set(1, 2, false),
            Ok(())
        );

        assert_eq!(
            world.get(1, 2),
            Ok(false)
        );
    }

    #[test]
    fn get_invalid() {
        let world = World::new(3);

        assert!(world.get(1,3).is_err())
    }

    #[test]
    fn set_invalid() {
        let mut world = World::new(3);
        
        assert!(world.set(1, 3, true).is_err())
    }

    #[test]
    fn randomise() {
        let world = World::new(100);
        let mut world_random = world.clone();

        world_random.randomise().unwrap();

        assert_ne!(
            world.elements,
            world_random.elements
        );
    }

    #[test]
    fn step() {
        // this test also covers get_score
        //
        //  0 1 0        1 0 0
        //  1 1 0   =>   0 1 1
        //  0 0 1        0 1 0
        //
        let mut world = World::new(3);

        world.set(0, 1, true).unwrap();
        world.set(1, 0, true).unwrap();
        world.set(1, 1, true).unwrap();
        world.set(2, 2, true).unwrap();

        world.step().unwrap();

        let answers: Vec<u8> = vec![1, 0, 0, 0, 1, 1, 0, 1, 0];
        for row in 0..world.size {
            for col in 0..world.size {
                assert_eq!(
                    world.get(row, col).unwrap() as u8,
                    answers[row*world.size + col]
                );
            }
        } 
    }

    #[test]
    fn _get_vector_index() {
        let world = World::new(3);

        assert_eq!(
            world._get_vector_index(2, 1),
            11
        );

        assert_eq!(
            world._get_vector_index(0, 2),
            2
        );

        let world = World::new(8);

        assert_eq!(
            world._get_vector_index(5, 7),
            57
        );

        assert_eq!(
            world._get_vector_index(2, 8),
            28
        );
    }

    #[test]
    fn _get_and_set_unmapped() {
        let mut world = World::new(3);

        let value = world._get_unmapped(2, 2);
        world._set_unmapped(2, 2, !value);

        assert_eq!(
            !value,
            world._get_unmapped(2, 2)
        );

        assert_eq!(
            !value,
            world.get(1,1).unwrap()
        );
    }
}