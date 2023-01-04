use tools;
use regex::Regex;

fn main() {
    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());

    println!("Solution: {}", solve_star_1(input));
}

fn solve_star_1<I>(input: I) -> usize where I: IntoIterator<Item = String>
{
    let file_size_regex = Regex::new("\\d+").unwrap();
    let max: usize = 100000;

    let mut dir_sizes: Vec<usize> = vec![0];
    let mut sum: usize = 0;

    println!("{:?}", dir_sizes);
    
    for line in input {
        let components = line.split(' ').collect::<Vec<_>>();
        assert!(components.len() >= 2, "Too few components in line '{}'", line);
        
        match components[0] {
            "$" => {
                match components[1] {
                    "cd" => {
                        match components[2] {
                            "/" => {
                                dir_sizes.drain(1..);
                            },
                            ".." => {
                                let cur_size = dir_sizes.pop().unwrap();
                                
                                if cur_size <= max {
                                    println!("+ {}", cur_size);
                                    sum += cur_size;
                                }
                                
                                if let Some(x) = dir_sizes.last_mut() {
                                    *x += cur_size;
                                }
                            },
                            _ => { // change to subdirectory
                                dir_sizes.push(0);
                            }
                        }
                    },
                    _ => {} // ignore
                }
            },
            size if file_size_regex.is_match(size) => {
                if let Some(x) = dir_sizes.last_mut() {
                    *x += size.parse::<usize>().unwrap();
                }
            },
            _ => {} // ignore
        }

        println!("{:?}", dir_sizes);
    }

    // Please note: the solution should fail if the last visited directory is <= max.

    sum
}

#[cfg(test)]
mod tests
{
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_example_star_1()
    {
        let input_path = "res/example_input.txt";
        let input = tools::read_lines(input_path)
            .expect("Should find puzzle input")
            .map(|x| x.unwrap());

        assert_eq!(solve_star_1(input), 95437);
    }
}
