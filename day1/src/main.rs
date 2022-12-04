use std::env;
use std::fs;
use std::process;
use std::error::Error;

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
    let food_contents = fs::read_to_string(config.food_file_path)?;
    let food_sets: Vec<&str> = food_contents.split("\n\n").collect();
    let mut elves: Vec<Elf> = vec![];

    for (idx, set) in food_sets.iter().enumerate() {
        let mut total_calorie_count = 0;
        let food_items: Vec<&str> = set.split("\n").collect();
        for item in food_items {
            let calorie_count: i32 = item.parse().unwrap();
            total_calorie_count += calorie_count;
        }
        let elf_id: i32 = idx.try_into().unwrap();
        let elf = Elf { elf_id, total_calorie_count };
        elves.push(elf);
    }

    let mut largest_elf = 0;

    for (idx, elf) in elves.iter().enumerate() {
        if elves[largest_elf].total_calorie_count < elf.total_calorie_count {
            largest_elf = idx.try_into().unwrap();
        }
        println!("Elf {} has {} calories", elf.elf_id, elf.total_calorie_count);
    }

    println!("The elf with the most calories is elf #{}", largest_elf);

    Ok(())
}

struct Elf {
    elf_id: i32,
    total_calorie_count: i32
}

struct Config {
    food_file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: advent_of_code_22 <food data file>")
        }
        let food_file_path = args[1].clone();
        Ok(Config { food_file_path })
    }
}
