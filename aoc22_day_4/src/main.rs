use tools;

fn main() {
    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());
    
        println!("The result is {}", solve_star_2(input));
}

struct Segment { left: i32, right: i32 }

#[allow(dead_code)]
fn solve_star_1<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    solve_star(input, |s1, s2| {
        s1.left <= s2.left && s1.right >= s2.right || 
        s2.left <= s1.left && s2.right >= s1.right
    })
}

#[allow(dead_code)]
fn solve_star_2<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    solve_star(input, |s1, s2| {
        s1.right >= s2.left && s1.left <= s2.right
    })
}

fn solve_star<I>(input: I, f: fn(Segment, Segment) -> bool) -> i32 where I: IntoIterator<Item = String>
{
    let mut result = 0;

    for line in input.into_iter() {
        let camp_sections = line.split(',').into_iter().map(|camp_section_range| {
            camp_section_range.split('-').map(|camp_section| camp_section.parse::<i32>().unwrap()).collect::<Vec<_>>()
        }).flatten().collect::<Vec<_>>();

        assert_eq!(camp_sections.len(), 4);
        let s1 = Segment { left: camp_sections[0], right: camp_sections[1] };
        let s2 = Segment { left: camp_sections[2], right: camp_sections[3] };

        if f(s1, s2) {
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
        let expected_contained_segments = solve_star_1(input);
        assert_eq!(expected_contained_segments, 2);
    }

    #[test]
    fn test_example_star_2()
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
        let expected_overlapping_segments = solve_star_2(input);
        assert_eq!(expected_overlapping_segments, 4);
    }
}
