use rust_solutions::solutions;
use rust_solutions::utils;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Expect path as first arg.");
    let num_iters = args.get(2).map_or(20, |s| s.parse().unwrap());

    match fs::read_to_string(path) {
        Ok(content) => {
            utils::run_benchmark(|| solutions::day6::sol2(&content), num_iters);
        }
        Err(e) => eprintln!("Failed to read file {path}: {e}"),
    }
}
