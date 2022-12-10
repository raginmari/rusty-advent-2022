use tools;

fn main() {
    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());
    
        println!("The result is {}", solve_star_1(input));
}

fn solve_star_1<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    let mut result = 0;

    for line in input.into_iter() {
        let camp_sections = line.split(',').into_iter().map(|camp_section_range| {
            camp_section_range.split('-').map(|camp_section| camp_section.parse::<i32>().unwrap()).collect::<Vec<_>>()
        }).flatten().collect::<Vec<_>>();

        assert_eq!(camp_sections.len(), 4);
        let (left_1, left_2, right_1, right_2) = 
            (camp_sections[0], camp_sections[2], camp_sections[1], camp_sections[3]);

        if (left_1 <= left_2 && right_1 >= right_2) || (left_2 <= left_1 && right_2 >= right_1) {
            result += 1;
        }
    }

    result
}

mod tests
{
    #[allow(unused_imports)]   
    use super::*;

    #[test]
    fn test_example_star_1()
    {
        let lines = vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8"
        ];
        let input = lines.iter().map(|x| x.to_string());
        let expected_sum_of_priorities = solve_star_1(input);
        assert_eq!(expected_sum_of_priorities, 2);
    }
}
