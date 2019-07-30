use std::env;
use std::process;
use std::thread;
use std::time;

use game_of_life::World;

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

struct Config {
    size: usize,
    iterations: u32,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.")
        }

        let size = args[1].parse::<usize>().unwrap();
        let iterations = args[2].parse::<u32>().unwrap();

        Ok(Config { size, iterations })
    }
}

fn run(config: Config) -> Result<(), &'static str> {
    let mut world = World::new(config.size);
    world.randomise()?;

    for _ in 0..config.iterations {
        // clear console output
        print!("{}[2J", 27 as char);

        world.step()?;

        world.display();

        if world.is_empty() {
            println!("World is empty. Ending early.");
            break;
        } else {
            thread::sleep(time::Duration::from_millis(1000));
        }
    }

    Ok(())
}