use anyhow::{bail, Error};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Vec<(u32, Color)>>,
}

fn color_red(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("red")(input)?;
    Ok((input, Color::Red))
}

fn color_green(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("green")(input)?;

    Ok((input, Color::Green))
}

fn color_blue(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("blue")(input)?;
    Ok((input, Color::Blue))
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((color_red, color_green, color_blue))(input)
}

fn integer(input: &str) -> IResult<&str, u32> {
    // NOTE will panic on i32 overflow
    let (input, val) = nom::character::complete::digit1(input)?;
    let val = u32::from_str_radix(val, 10).unwrap();
    Ok((input, val))
}

fn count_color_pair(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, count) = integer(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = color(input)?;

    Ok((input, (count, color)))
}

fn game_is_possible(game: &Game) -> bool {
    for pull in &game.pulls {
        let mut red = 12;
        let mut green = 13;
        let mut blue = 14;
        for (count, color) in pull {
            match color {
                Color::Red => {
                    if *count >= red {
                        return false;
                    }
                    red -= count;
                }
                Color::Green => {
                    if *count >= green {
                        return false;
                    }
                    green -= count;
                }
                Color::Blue => {
                    if *count >= blue {
                        return false;
                    }
                    blue -= count;
                }
            }
        }
    }
    true
}
fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = integer(input)?;
    let (input, _) = tag(": ")(input)?;
    let inner_list = separated_list1(tag("; "), count_color_pair);
    let mut outer_list = separated_list1(tag(", "), inner_list);

    let (input, pulls) = outer_list(input)?;

    Ok((input, Game { id, pulls }))
}

fn main() -> Result<(), Error> {
    let mut games = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line?;

        let game = match terminated(game, eof)(&line) {
            Ok((_rest, game)) => game,
            Err(e) => bail!("parsing failed: {e:?}"),
        };

        // println!("{game:?}");
        games.push(game);
    }

    let mut sum = 0;
    for game in &games {
        if game_is_possible(game) {
            println!("possible: {game:?}");
            sum += game.id;
        } else {
            println!("not possible: {game:?}");
        }
    }

    println!("{sum}");

    Ok(())
}
