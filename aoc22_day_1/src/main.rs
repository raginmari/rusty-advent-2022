use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

fn main()
{
    // TIL: file paths are specified relative to the folder where Cargo.toml is located.
    let input_path = "res/input.txt";
    let input = read_lines(input_path).expect("Should find puzzle input");
    
    solve(input, 3);
}

// TIL: 'traits' are (kind of) Rust's protocols (Swift) or interfaces (C#).
fn solve<T>(input: Lines<T>, top: usize) where T: BufRead
{
    let mut calories_per_elf = Vec::new();
    let mut acc = 0;
    for maybe_line in input {
        let line = maybe_line.unwrap();
        if line.is_empty() {
            calories_per_elf.push(acc);
            acc = 0;
        } else {
            // TIL: use 'turbofish' ::<i32> or the explicit type i32 to tell the compiler what to parse / the generic type.
            let calories = line.parse::<i32>().unwrap();
            acc += calories;
        }
    }

    // Don't forget the last elf
    calories_per_elf.push(acc);

    calories_per_elf.sort_unstable();
    let total_calories = calories_per_elf.iter().rev().take(top).sum::<i32>();

    println!("The top {0} elves carry {1} calories", top, total_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
