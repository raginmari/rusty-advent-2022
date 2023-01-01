use std::{collections::HashSet};
use tools;

fn main() {
    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap())
        .take(1)
        .collect::<String>();

    println!("Solution: {}", solve_star_2(input));    
}

#[allow(dead_code)]
fn solve_star_1(signal: String) -> usize
{
    solve(signal, 4)
}

#[allow(dead_code)]
fn solve_star_2(signal: String) -> usize
{
    solve(signal, 14)
}

fn solve(signal: String, marker_length: usize) -> usize
{
    let slice = &signal.chars().collect::<Vec<_>>()[..];
    for window in slice.windows(marker_length).enumerate() {
        let distinct_characters = window.1.iter().collect::<HashSet<_>>();
        if distinct_characters.len() == marker_length {
            return window.0 + marker_length
        }
    }

    panic!("reached end of signal without result")
}

#[cfg(test)] // Compiled only when running tests
mod tests
{
    #[allow(unused_imports)]
    use super::*;
    use ntest::test_case;
    
    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_example_star_1(signal: &str, expected_result: usize)
    {
        assert_eq!(solve_star_1(String::from(signal)), expected_result);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_example_star_2(signal: &str, expected_result: usize)
    {
        assert_eq!(solve_star_2(String::from(signal)), expected_result);
    }
}