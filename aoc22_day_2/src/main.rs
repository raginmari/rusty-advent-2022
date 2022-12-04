use tools;

fn main() 
{
    let input_path = "res/input.txt";
    let input = tools::read_lines(input_path)
        .expect("Should find puzzle input")
        .map(|x| x.unwrap());
    
    println!("The total score is {}", solve_star_2(input));
}

// Copy performs bit-wise copying. All constituents of Copy-able types must be Copy as well.
// https://doc.rust-lang.org/stable/std/marker/trait.Copy.html

// Clone defines a function that performs a type-specific clone. Clone is parent of Copy. Potentially more costly than Copy.
// https://doc.rust-lang.org/stable/std/clone/trait.Clone.html

#[derive(PartialEq, Clone, Copy)]
enum Shape 
{
    Rock,
    Paper,
    Scissors
}

impl Shape
{
    fn from_char(s: char) -> Shape 
    {
        match s {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!()
        }
    }

    fn get_defeats(&self) -> Shape
    {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    fn get_defeated_by(&self) -> Shape 
    {
        match *self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }
}

#[allow(dead_code)]
// TIL: requiring IntoIterator makes it possible to call the function with Vec as well as an Iterator
fn solve_star_1<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    input.into_iter().map(|line| {
        let bytes = line.as_bytes();
        let opponent = Shape::from_char(bytes[0] as char);
        let me = Shape::from_char(bytes[2] as char);
        value_of_shape(&me) + points_for(&opponent, &me)
    }).sum()
}

#[allow(dead_code)]
fn solve_star_2<I>(input: I) -> i32 where I: IntoIterator<Item = String>
{
    input.into_iter().map(|line| {
        let bytes = line.as_bytes();
        let opponent = Shape::from_char(bytes[0] as char);
        let me = match bytes[2] as char {
            'X' => opponent.get_defeats(),
            'Y' => opponent, // Performs copy because Shape is Copy
            'Z' => opponent.get_defeated_by(),
            _ => panic!()
        };
        value_of_shape(&me) + points_for(&opponent, &me)
    }).sum()
}

fn value_of_shape(shape: &Shape) -> i32
{
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    } // TIL: using a semicolon here ends the expression / removes the 'tail' and requires an explicit return
}

fn points_for(opponent: &Shape, me: &Shape) -> i32
{
    if me == opponent { return 3 }
    if me.get_defeats() == *opponent { 6 } else { 0 }
}

mod tests
{
    // TIL: Uses names from parent scope including private symbols
    #[allow(unused_imports)]   
    use super::*;

    #[test]
    fn test_example_star_1()
    {
        let lines = vec!["A Y", "B X", "C Z"];
        let input = lines.iter().map(|x| x.to_string());
        let score = solve_star_1(input);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_example_star_2()
    {
        let lines = vec!["A Y", "B X", "C Z"];
        let input = lines.iter().map(|x| x.to_string());
        let score = solve_star_2(input);
        assert_eq!(score, 12);
    }
}