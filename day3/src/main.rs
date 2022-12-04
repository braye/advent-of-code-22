use std::env;
use std::fs;
use std::process;
use std::error::Error;
// use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.rucksack_contents_path)?;
    let rucksacks = contents.split("\n").collect::<Vec<&str>>();
    let mut total_priority = 0;

    for rucksack in rucksacks {
        let mut matching_character = ' ';
        let compartment_size = rucksack.len() / 2;
        let (compartment_1, compartment_2) = rucksack.split_at(compartment_size);
        println!("compartment size: {}", compartment_size);

        for search_char in compartment_1.chars() {
            if compartment_2.contains(search_char) {
                matching_character = search_char;
                println!("Found matching character in {}: {}", rucksack, matching_character);
                break;
            }
        }
        // crude but effective
        let character_priority_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let character_priority_value = character_priority_list.find(matching_character).unwrap() + 1;
        total_priority += character_priority_value;
    }

    println!("Total priority value: {}", total_priority);
    
    Ok(())
}

struct Config {
    rucksack_contents_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: day3 <rucksack contents>")
        }
        let rucksack_contents_path = args[1].clone();
        Ok(Config { rucksack_contents_path })
    }
}
