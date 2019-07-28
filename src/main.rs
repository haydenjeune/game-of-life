use game_of_life::World;

fn main() {
    let mut world = World::new(3);
    world.randomise().unwrap();

    println!("Size: {}", world.size);
    println!("Elements:");
    world.display();

    world.step().unwrap();

    println!("Elements:");
    world.display();
    
}