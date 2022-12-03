use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_path = "res/input.txt";
    let input = read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());
    
    println!("The total score is {}", solve(input));
}

#[derive(PartialEq)]
enum Shape 
{
    Rock,
    Paper,
    Scissors
}

impl Shape
{
    fn from_char(s: &char) -> Shape 
    {
        match s {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!()
        }
    }
}

// TIL: requiring IntoIterator makes it possible to call the function with Vec as well as an Iterator
fn solve<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    input.into_iter().map(|line| {
        let bytes = line.as_bytes();
        let opponent = Shape::from_char(&(bytes[0] as char));
        let me = Shape::from_char(&(bytes[2] as char));
        value_of_symbol(&me) + points_for(&opponent, &me)
    }).sum()
}

fn value_of_symbol(symbol: &Shape) -> i32
{
    match symbol {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    } // TIL: using a semicolon here ends the expression / removes the 'tail' and requires an explicit return
}

fn points_for(opponent: &Shape, me: &Shape) -> i32
{
    match opponent {
        // Lose
        Shape::Rock if me == &Shape::Scissors => 0,
        Shape::Paper if me == &Shape::Rock => 0,
        Shape::Scissors if me == &Shape::Paper => 0,
        
        // Win
        Shape::Rock if me == &Shape::Paper => 6,
        Shape::Paper if me == &Shape::Scissors => 6,
        Shape::Scissors if me == &Shape::Rock => 6,

        // Draw
        _ => 3
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

mod tests
{
    // TIL: Uses names from parent scope including private symbols
    #[allow(unused_imports)]   
    use super::*;

    #[test]
    fn test_example_input()
    {
        let lines = vec!["A Y", "B X", "C Z"];
        let input = lines.iter().map(|x| x.to_string());
        let score = solve(input);
        assert_eq!(score, 15);
    }
}