use nom::{
    IResult, Parser,
    character::complete::{newline, u32},
    combinator::opt,
    multi::many1,
};

const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

const PUZZLE_INPUT: &str = include_str!("../inputs/one.pzlin");

fn parse_one_line(input: &str) -> IResult<&str, u32> {
    let (input, number) = u32(input)?;
    let (input, _) = opt(newline).parse(input)?;
    Ok((input, number))
}

fn parse_one_group(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = many1(parse_one_line).parse(input)?;
    let (input, _) = opt(newline).parse(input)?;
    Ok((input, numbers))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(parse_one_group).parse(input)
}

fn do_puzzle(inputs: Vec<Vec<u32>>) -> u32 {
    inputs
        .into_iter()
        .map(|group| group.iter().sum())
        .max()
        .unwrap_or(0)
}

fn do_puzzle_extra(inputs: Vec<Vec<u32>>) -> Vec<u32> {
    let mut middle = inputs
        .into_iter()
        .map(|group| group.iter().sum())
        .collect::<Vec<u32>>();

    // note(teesh): hey rust, WHY IS THIS INPLACE???
    middle.sort();
    middle.reverse();

    vec![middle[0], middle[1], middle[2]]
}

#[test]
fn parse_example_input() {
    let expected = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];

    let result = parse_input(EXAMPLE_INPUT);

    assert!(result.is_ok(), "Parsing failed: {:?}", result);
    let (remaining, parsed) = result.unwrap();

    assert!(
        remaining.is_empty(),
        "Unparsed input remaining: '{}'",
        remaining
    );
    assert_eq!(parsed, expected);
}

#[test]
fn do_example_input() {
    // note(teesh): we know this is safe, look at the last test
    let (_, groups) = parse_input(EXAMPLE_INPUT).unwrap();

    let output = do_puzzle(groups);
    assert_eq!(output, 24000);
}

#[test]
fn do_puzzle_input() {
    let (_, groups) = parse_input(PUZZLE_INPUT).unwrap();

    let output = do_puzzle(groups);
    assert_eq!(output, 72017);
}

#[test]
fn do_extra_credit() {
    let (_, groups) = parse_input(PUZZLE_INPUT).unwrap();

    let output = do_puzzle_extra(groups);
    assert_eq!(output, vec![72017, 71144, 69359]);
    assert_eq!(output.iter().sum::<u32>(), 212520);
}
