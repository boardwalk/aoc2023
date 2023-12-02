use anyhow::{bail, Error};
use std::io::stdin;

// for part one, just remove the english digit elements from this array
static STR_TO_DIGIT: [(&str, usize); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() -> Result<(), Error> {
    let mut sum = 0;
    for line in stdin().lines() {
        let line = line?;

        let mut digits: Vec<(usize, usize)> = Vec::new(); // (index of the start of the digit on the line, value of the digit)

        for (s, val) in &STR_TO_DIGIT {
            let mut start_idx = 0;
            while let Some(idx) = line[start_idx..].find(s) {
                digits.push((idx + start_idx, *val));

                start_idx += idx + s.len();
            }
        }

        if digits.is_empty() {
            bail!("no digits on line");
        }

        let (_idx, first) = digits.iter().min_by_key(|(idx, _val)| idx).unwrap();
        let (_idx, last) = digits.iter().max_by_key(|(idx, _val)| idx).unwrap();

        println!("{digits:?}");

        let val = first * 10 + last;
        sum += val;
    }

    println!("{sum}");

    Ok(())
}
