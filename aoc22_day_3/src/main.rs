use std::collections::HashSet;
use itertools::{self, Itertools};
use tools;

fn main() {
    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());
    
        println!("The sum of priorities is {}", solve_star_2(input));
}

#[allow(dead_code)]
fn solve_star_1<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    input.into_iter().map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        let set_a = a.chars().collect::<HashSet<char>>();
        let set_b = b.chars().collect::<HashSet<char>>();
        let duplicate = set_a.intersection(&set_b).next().expect("intersection should not be empty");
        priority_of(duplicate)
    }).sum()
}

#[allow(dead_code)]
fn solve_star_2<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    input.into_iter().chunks(3).into_iter().map(|backpacks| {
        let sorted_items = backpacks.map(|x| x.chars().collect::<HashSet<_>>()).flatten().sorted().collect::<Vec<_>>();
        for i in 2..sorted_items.len() {
            if sorted_items[i - 2] == sorted_items[i - 1] && sorted_items[i - 1] == sorted_items[i] {
                return priority_of(&sorted_items[i])
            }
        }

        panic!("each triplet of backpacks must contain one shared item");
    }).sum()
}

fn priority_of(c: &char) -> i32
{
    let ascii = *c as i32;

    match ascii {
        // a-z
        97 ..= 122 => ascii - 97 + 1,
        
        // A-Z
        65 ..=  90 => ascii - 65 + 27,

        _ => panic!("character should be an ASCII letter")
    }
}

mod tests
{
    #[allow(unused_imports)]   
    use super::*;

    #[test]
    fn test_example_star_1()
    {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ];
        let input = lines.iter().map(|x| x.to_string());
        let expected_sum_of_priorities = solve_star_1(input);
        assert_eq!(expected_sum_of_priorities, 157);
    }

    #[test]
    fn test_example_star_2()
    {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ];
        let input = lines.iter().map(|x| x.to_string());
        let expected_sum_of_priorities = solve_star_2(input);
        assert_eq!(expected_sum_of_priorities, 70);
    }
}