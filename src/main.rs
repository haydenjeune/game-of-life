use std::env;
use std::process;
use std::thread;
use std::time;

use game_of_life::World;
use game_of_life::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
    
}

fn run(config: Config) -> Result<(), &'static str> {
    let mut world = World::new(config.size);
    world.randomise()?;

    for _ in 0..config.iterations {
        clear_terminal();

        world.step()?;

        world.display();

        if world.is_empty() {
            clear_terminal();
            println!("World is empty. Ending early.");
            break;
        } else {
            thread::sleep(time::Duration::from_millis(config.pause_ms));
        }
    }
    Ok(())
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}