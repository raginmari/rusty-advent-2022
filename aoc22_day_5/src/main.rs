use tools;
use domain::{Ship, ContainerStack};
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let initial_stacks = vec![
        "SZPDLBFC",
        "NVGPHWB",
        "FWBJG",
        "GJNFLWCS",
        "WJLTPMSH",
        "BCWGFS",
        "HTPMQBW",
        "FSWT",
        "NCR"
    ];

    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());
    
    let ship = create_ship(&initial_stacks);
    println!("The result is {}", solve_star_2(ship, input));
}

#[derive(Debug, Copy, Clone)]
struct Step
{
    count: usize,
    from_stack: usize,
    to_stack: usize
}

#[allow(dead_code)]
fn solve_star_1<I>(mut ship: Ship, step_strings: I) -> String where I: IntoIterator<Item = String>
{
    let steps = step_strings.into_iter().map(|x| create_step(&x));

    for step in steps {
        for _ in 0..step.count {
            let to_move = ship.container_stacks[step.from_stack].unload_one().unwrap();
            ship.container_stacks[step.to_stack].load(to_move)
        }
    }

    ship.top_containers()
}

#[allow(dead_code)]
fn solve_star_2<I>(mut ship: Ship, step_strings: I) -> String where I: IntoIterator<Item = String>
{
    let steps = step_strings.into_iter().map(|x| create_step(&x));

    for step in steps {
        let mut to_move = ship.container_stacks[step.from_stack].unload_many(step.count);
        ship.container_stacks[step.to_stack].load_many(&mut to_move);
    }

    ship.top_containers()
}

fn create_ship(stacks: &Vec<&str>) -> Ship
{
    let mut ship = Ship { container_stacks: Vec::new() };

    for stack in stacks {
        let containers = stack.chars().collect::<Vec<_>>();
        let container_stack = ContainerStack::new(containers);
        ship.container_stacks.push(container_stack);
    }

    ship
}

fn create_step(step_string: &str) -> Step
{
    lazy_static! {
        static ref STEP_REGEX: Regex = Regex::new("move (\\d+) from (\\d) to (\\d)").unwrap();
    }

    let captures = STEP_REGEX.captures(step_string).unwrap();
    assert_eq!(captures.len(), 1 + 3, "missing captures");
    
    Step {
        count: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        from_stack: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
        to_stack: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1
    }
}

mod domain
{
    #[derive(Debug)]
    pub struct Ship
    {
        // TODO: This field should be hidden
        pub container_stacks: Vec<ContainerStack>
    }

    impl Ship 
    {
        pub fn top_containers(&self) -> String
        {
            self.container_stacks.iter().map(|x| x.containers.last().unwrap_or(&'\0')).collect::<String>()
        }
    }

    #[derive(Debug)]
    pub struct ContainerStack
    {
        containers: Vec<char>
    }

    impl ContainerStack
    {
        pub fn new(containers: Vec<char>) -> ContainerStack
        {
            ContainerStack { containers }
        }

        pub fn unload_one(&mut self) -> Option<char>
        {
            self.containers.pop()
        }

        pub fn unload_many(&mut self, count: usize) -> Vec<char>
        {
            let i = self.containers.len() - count;
            self.containers.drain(i..).collect::<Vec<_>>()
        }

        pub fn load(&mut self, container: char)
        {
            self.containers.push(container);
        }

        pub fn load_many(&mut self, containers: &mut Vec<char>)
        {
            self.containers.append(containers);
        }
    }
}

mod tests
{
    #[allow(unused_imports)]   
    use super::*;

    #[test]
    fn test_example_star_1()
    {
        let stacks = vec![
            "ZN",
            "MCD",
            "P"
        ];

        let steps = vec![
            String::from("move 1 from 2 to 1"),
            String::from("move 3 from 1 to 3"),
            String::from("move 2 from 2 to 1"),
            String::from("move 1 from 1 to 2")
        ];
        
        let ship = create_ship(&stacks);
        let expected_top_containers = solve_star_1(ship, steps);
        assert_eq!(expected_top_containers, "CMZ");
    }

    #[test]
    fn test_example_star_2()
    {
        let stacks = vec![
            "ZN",
            "MCD",
            "P"
        ];

        let steps = vec![
            String::from("move 1 from 2 to 1"),
            String::from("move 3 from 1 to 3"),
            String::from("move 2 from 2 to 1"),
            String::from("move 1 from 1 to 2")
        ];
        
        let ship = create_ship(&stacks);
        let expected_top_containers = solve_star_2(ship, steps);
        assert_eq!(expected_top_containers, "MCD");
    }
}
