use std::collections::HashSet;
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
    let mut backpacks: Vec<String> = Vec::new();
    input.into_iter().map(|line| {
        
        backpacks.push(line);

        if backpacks.len() < 3 {
            return 0 // Do not add to the sum
        }
        
        let set_a = backpacks[0].chars().collect::<HashSet<char>>();
        let set_b = backpacks[1].chars().collect::<HashSet<char>>();
        let set_c = backpacks[2].chars().collect::<HashSet<char>>();
        backpacks.clear();

        let int_1 = set_a.intersection(&set_b).collect::<HashSet<&char>>();
        let int_2 = set_a.intersection(&set_c).collect::<HashSet<&char>>();
        let badge = **int_1.intersection(&int_2).next().expect("intersection should not be empty");

        priority_of(&badge)
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