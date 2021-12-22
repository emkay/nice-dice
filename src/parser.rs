use crate::dice;
use nom::{
    IResult,
    character::complete::{one_of, char},
    multi::{many1, many0}
};

use nom::sequence::separated_pair;

pub fn parse(s: &str) -> dice::Roll {
    let parsed = parse_roll(s);

    let d6 = dice::Die::new(parsed.sides);
    let roll = d6.roll(parsed.num_dice);
    roll
}

fn digit(d: &str) ->  IResult<&str, char> {
    one_of("0123456789")(d)
}

fn match_d(c: &str) -> IResult<&str, char> {
    char('d')(c)
}

#[derive(Debug)]
struct ParsedRoll {
    num_dice: u16,
    sides: u16
}

fn parse_roll(input: &str) -> ParsedRoll {
    let mut parser = separated_pair(many0(digit), match_d, many1(digit));
    let (_, (d1, d2)) = parser(input).expect("Could not parse dice syntax.");

    let num_dice_s: String = d1.into_iter().collect();
    let sides_s: String = d2.into_iter().collect();

    let num_dice = num_dice_s.parse::<u16>().unwrap_or(1);
    let sides = sides_s.parse::<u16>().expect("Could not parse sides!");

    ParsedRoll {
        num_dice,
        sides
    }
}

#[test]
fn parse_roll_test() {
    let pr = parse_roll("d6");
    assert_eq!(pr.num_dice, 1);
    assert_eq!(pr.sides, 6);

    let pr = parse_roll("2d12");
    assert_eq!(pr.num_dice, 2);
    assert_eq!(pr.sides, 12);

    let pr = parse_roll("12d12");
    assert_eq!(pr.num_dice, 12);
    assert_eq!(pr.sides, 12);

    let pr = parse_roll("d20");
    assert_eq!(pr.num_dice, 1);
    assert_eq!(pr.sides, 20);

    let pr = parse_roll("4d20");
    assert_eq!(pr.num_dice, 4);
    assert_eq!(pr.sides, 20);
}
