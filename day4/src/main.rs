use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::fmt;

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
    let contents = fs::read_to_string(config.assignments_path)?;
    let mut pairs: Vec<(ElfAssignment, ElfAssignment)> = Vec::new();
    let parsed_pairs = contents.split("\n").collect::<Vec<&str>>();

    for pair in parsed_pairs {
        let elves = pair.split(",").collect::<Vec<&str>>();
        let elf1 = parse_assignment(&elves[0]);
        let elf2 = parse_assignment(&elves[1]);
        let elf_assignment_1 = ElfAssignment{ start: elf1.0, end: elf1.1 };
        let elf_assignment_2 = ElfAssignment{ start: elf2.0, end: elf2.1 };
        pairs.push((elf_assignment_1, elf_assignment_2));
    }

    let mut total_contained_assignments = 0;
    let mut total_overlapping_assignments = 0;
    for elves in pairs {
        // find any assignments fully contained within one another
        if elves.0.start >= elves.1.start && elves.0.end <= elves.1.end {
            total_contained_assignments += 1;
        } else if elves.1.start >= elves.0.start && elves.1.end <= elves.0.end {
            total_contained_assignments += 1;
        }

        // find assignments that overlap at all
        if elves.0.start <= elves.1.start && elves.0.end >= elves.1.start {
            total_overlapping_assignments += 1;
        } else if elves.1.start <= elves.0.start && elves.1.end >= elves.0.start {
            total_overlapping_assignments += 1;
        }
    }

    println!("total assignments contained within one another: {}", total_contained_assignments);
    println!("total overlapping assignments: {}", total_overlapping_assignments);

    Ok(())
}

fn parse_assignment(assignment: &str) -> (i32, i32) {
    let assginment_split = assignment.split("-").collect::<Vec<&str>>();
    return (assginment_split[0].parse().unwrap(), assginment_split[1].parse().unwrap());
}

struct ElfAssignment {
    start: i32,
    end: i32
}

impl fmt::Display for ElfAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ElfAssignment:{{ start: {}, end: {} }}", self.start, self.end)
    }
}

struct Config {
    assignments_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: day4 <assignments file>")
        }
        let assignments_path = args[1].clone();
        Ok(Config { assignments_path })
    }
}
