use anyhow::{bail, Error};
use std::io::stdin;

fn main() -> Result<(), Error> {
    let mut sum = 0;
    for line in stdin().lines() {
        let line = line?;

        let digits = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        if digits.is_empty() {
            bail!("missing any digits on line");
        }

        let first = digits.first().copied().unwrap();
        let last = digits.last().copied().unwrap();

        sum += first * 10 + last;
    }

    println!("{sum}");
    Ok(())
}
