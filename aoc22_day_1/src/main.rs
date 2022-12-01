use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

fn main()
{
    // TIL: file paths are specified relative to the folder where Cargo.toml is located.
    let input_path = "res/input.txt";
    let input = read_lines(input_path).expect("Should find puzzle input");
    solve_1(input);
}

// TIL: 'traits' are (kind of) Rust's protocols (Swift) or interfaces (C#).
fn solve_1<T>(input: Lines<T>) where T: BufRead
{
    let mut max_calories = 0;
    let mut acc = 0;
    for maybe_line in input {
        let line = maybe_line.unwrap();
        if line.is_empty() {
            max_calories = max(max_calories, acc);
            acc = 0;
        } else {
            // TIL: use 'turbofish' ::<i32> or the explicit type i32 to tell the compiler what to parse / the generic type.
            let calories = line.parse::<i32>().unwrap();
            acc += calories;
        }
    }

    // Don't forget the last elf
    max_calories = max(max_calories, acc);
    
    println!("The elf carrying the most calories has {} calories", max_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
