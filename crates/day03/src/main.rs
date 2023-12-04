use anyhow::{anyhow, bail, Error};
use std::collections::HashSet;

struct Schematic {
    rows: Vec<String>,
    width: usize,
}

fn is_symbol(val: u8) -> bool {
    if val.is_ascii_digit() {
        return false;
    }
    if val == b'.' {
        return false;
    }
    true
}

impl Schematic {
    fn new(rows: Vec<String>) -> Result<Self, Error> {
        let width = rows
            .get(0)
            .ok_or_else(|| anyhow!("no rows in schematic"))?
            .len();

        if !rows.iter().all(|row| row.len() == width) {
            bail!("row widths do not all match");
        }
        Ok(Self { rows, width })
    }

    fn rows(&self) -> Result<i32, Error> {
        i32::try_from(self.rows.len()).map_err(|e| e.into())
    }

    fn cols(&self) -> Result<i32, Error> {
        i32::try_from(self.width).map_err(|e| e.into())
    }

    fn get(&self, row: i32, col: i32) -> Option<u8> {
        let Ok(row_idx) = usize::try_from(row) else {
            return None;
        };

        let Ok(col_idx) = usize::try_from(col) else {
            return None;
        };

        let Some(row) = self.rows.get(row_idx) else {
            return None;
        };

        let Some(val) = row.as_bytes().get(col_idx) else {
            return None;
        };

        Some(*val)
    }
    fn get_number(&self, row: i32, min_col: i32, max_col: i32) -> Result<i32, Error> {
        let mut digits = Vec::new();

        for col in min_col..=max_col {
            let val = self
                .get(row, col)
                .ok_or_else(|| anyhow!("failed to get value from schematic"))?;
            digits.push(val);
        }

        let digits_str = String::from_utf8(digits)?;

        let val = i32::from_str_radix(&digits_str, 10)?;

        Ok(val)
    }

    fn is_symbol_adjacent(&self, row: i32, col: i32) -> bool {
        for roff in -1..=1 {
            for coff in -1..=1 {
                let Some(val) = self.get(row + roff, col + coff) else {
                    continue;
                };
                if is_symbol(val) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> Result<(), Error> {
    let mut rows = Vec::new();
    for line in std::io::stdin().lines() {
        rows.push(line?);
    }

    let s = Schematic::new(rows)?;

    let mut part_num_positions: HashSet<(i32, i32)> = HashSet::new(); // (row, min col)

    for row in 0..s.rows()? {
        for col in 0..s.cols()? {
            if !s.get(row, col).unwrap().is_ascii_digit() {
                // not a digit, not a part number
                continue;
            }

            if !s.is_symbol_adjacent(row, col) {
                // symbol not adjacent, not a part number
                continue;
            }

            // if we get here, pos is pointed at a part number
            // problem is, we don't know where the number starts and ends
            // find the leftmost and rightmost digit from the current col
            println!("boop {row} {col}");
            let mut min_col = col;
            // let mut max_col = col;

            loop {
                let Some(val) = s.get(row, min_col - 1) else {
                    break;
                };

                if val.is_ascii_digit() {
                    min_col -= 1;
                } else {
                    break;
                }
            }

            // loop {
            //     let Some(val) = s.get(row, max_col + 1) else {
            //         break;
            //     };

            //     if val.is_ascii_digit() {
            //         max_col += 1;
            //     } else {
            //         break;
            //     }
            // }

            // println!("{min_col}, {max_col}");

            part_num_positions.insert((row, min_col));
        }
    }

    let mut sum = 0;

    for (row, min_col) in &part_num_positions {
        let mut max_col = *min_col;
        loop {
            let Some(val) = s.get(*row, max_col + 1) else {
                break;
            };

            if val.is_ascii_digit() {
                max_col += 1;
            } else {
                break;
            }
        }

        println!("{row} {min_col} {max_col}");

        let part_num = s.get_number(*row, *min_col, max_col)?;

        // println!("{part_num}");
        sum += part_num;
    }

    // println!("{part_num_positions:?}");
    println!("{sum}");
    Ok(())
}
