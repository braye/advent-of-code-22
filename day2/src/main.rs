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
    let mut score_from_guide = 0;
    let guide_contents = fs::read_to_string(config.strategy_guide_path)?;
    let battles: Vec<&str> = guide_contents.split("\n").collect();

    for battle in battles {
        let actions: Vec<&str> = battle.split_whitespace().collect();
        let opponent_action = determine_action(actions[0]);
        let player_action = determine_action(actions[1]);

        score_from_guide += match player_action {
            RPSAction::Rock => 1,
            RPSAction::Paper => 2,
            RPSAction::Scissors => 3
        };

        println!("Battle: {} vs {}", player_action, opponent_action);
        let result = determine_outcome(player_action, opponent_action);
        println!("Result: {} points", result);
        score_from_guide += result;
    }
    println!("Score from guide is {}", score_from_guide);

    Ok(())
}

fn determine_action(action: &str) -> RPSAction {
    return match action {
        "A" | "X" => RPSAction::Rock,
        "B" | "Y" => RPSAction::Paper,
        "C" | "Z" => RPSAction::Scissors,
        &_ => todo!()
    }
}

fn determine_outcome(player_action: RPSAction, opponent_action: RPSAction) -> i32 {
    // unholy code smell below
    return match (player_action, opponent_action) {
        (RPSAction::Rock, RPSAction::Rock) => 3,
        (RPSAction::Rock, RPSAction::Paper) => 0,
        (RPSAction::Rock, RPSAction::Scissors) => 6,

        (RPSAction::Paper, RPSAction::Rock) => 6,
        (RPSAction::Paper, RPSAction::Paper) => 3,
        (RPSAction::Paper, RPSAction::Scissors) => 0,

        (RPSAction::Scissors, RPSAction::Rock) => 0,
        (RPSAction::Scissors, RPSAction::Paper) => 6,
        (RPSAction::Scissors, RPSAction::Scissors) => 3,
    }
}

enum RPSAction {
    Rock,
    Paper,
    Scissors
}

impl fmt::Display for RPSAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            RPSAction::Rock => write!(f, "Rock"),
            RPSAction::Paper => write!(f, "Paper"),
            RPSAction::Scissors => write!(f, "Scissors")
        }
    }
}

struct Config {
    strategy_guide_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: day2 <strategy guide file>")
        }
        let strategy_guide_path = args[1].clone();
        Ok(Config { strategy_guide_path })
    }
}
