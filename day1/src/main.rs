use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::fmt;
use std::cmp::Ordering;

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

    elves.sort_unstable_by(|a, b| if a.total_calorie_count > b.total_calorie_count { Ordering::Less } else { Ordering::Greater });

    println!("The elf with the most calories is {}", elves[0]);
    let top_3_calories = elves[0].total_calorie_count + elves[1].total_calorie_count + elves[2].total_calorie_count;
    println!("the top 3 elves have {} calories", top_3_calories);

    Ok(())
}

struct Elf {
    elf_id: i32,
    total_calorie_count: i32
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Elf: id: {}, calories: {}", self.elf_id, self.total_calorie_count)
    }
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
