use rust_solutions::solutions;
use rust_solutions::utils;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        match fs::read_to_string(arg) {
            Ok(content) => {
                utils::run_benchmark(|| solutions::day5::sol(&content), 10);
            }
            Err(e) => eprintln!("Failed to read file {arg}: {e}"),
        }
    }
}
