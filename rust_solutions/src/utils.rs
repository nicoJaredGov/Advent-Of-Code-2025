use std::collections::HashSet;
use std::fmt::Display;
use std::io;
use std::time::Instant;

pub const EMPTY_LINE: &'static str = "\r\n\r\n";

pub fn read_filename_from_input() -> String {
    let mut file_name = String::new();

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");

    return file_name.trim().to_owned();
}

pub fn run_with_time_ms<T, U>(f: U) -> (T, String)
where
    T: Display,
    U: FnOnce() -> T,
{
    let before = Instant::now();
    let result = f();
    let elapsed = before.elapsed().as_millis().to_string();

    println!("\nResult: {result}\nSolution time: {elapsed} ms");

    (result, elapsed)
}

pub fn draw_grid(obstacles: &HashSet<(i32, i32)>, width: i32, height: i32) {
    for row in 0..height {
        let mut line = String::new();

        for col in 0..width {
            if obstacles.contains(&(col, row)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }

        println!("{line}");
    }
}
