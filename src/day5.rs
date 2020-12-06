use std::fs::File;
use std::io::prelude::*;

use crate::days;
use crate::err;

const PLANE_ROWS: u32 = 128;
const PLANE_COLS: u32 = 8;

#[derive(Debug)]
pub struct Day{}
impl days::Day for Day {
    fn run(&self) {
        println!("running day 5");
        let mut file = match File::open("data/05/input.txt") {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(e) => panic!(e),
            _ => {}
        };

        let plane = Plane::new(PLANE_ROWS, PLANE_COLS);
        let lines: Vec<&str> = contents.split("\n").map(|l| l.trim()).collect();

        let mut ids: Vec<u32> = Vec::new();
        for line in lines {
            match plane.find_seat(line) {
                Ok(seat) => ids.push(plane.seat_id(&seat)),
                Err(e) => println!("could not find seat: {:?}", e),
            }
        }
        ids.sort();

        let high_id = ids
            .iter()
            .fold(0, |acc, id| if *id > acc { *id } else { acc });
        let low_id = ids
            .iter()
            .fold(high_id, |acc, id| if *id < acc { *id } else { acc });
        println!("id range: {} - {}", low_id, high_id);

        for idx in 1..ids.len() - 1 {
            let id = ids[idx];
            let next_id = ids[idx + 1];

            if next_id != id + 1 {
                println!("missing id: {}", id + 1);
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn new(row: u32, col: u32) -> Seat {
        Seat { row: row, col: col }
    }
    fn row(&self) -> u32 {
        self.row
    }
    fn col(&self) -> u32 {
        self.col
    }
}

struct Plane {
    rows: u32,
    cols: u32,
}

impl Plane {
    fn new(rows: u32, cols: u32) -> Plane {
        Plane {
            rows: rows,
            cols: cols,
        }
    }

    fn find_seat(&self, seat: &str) -> Result<Seat, err::ParseError> {
        let clean_seat = seat.trim().to_uppercase();

        let seat_regex = regex::Regex::new(r"^[FB]{7}[RL]{3}$").unwrap();
        if !seat_regex.is_match(&clean_seat) {
            return Err(err::ParseError::new("invalid seat format", seat));
        }

        let mut row_front: f64 = 0.0;
        let mut row_back: f64 = (self.rows - 1) as f64;
        let mut col_left: f64 = 0.0;
        let mut col_right: f64 = (self.cols - 1) as f64;

        for p in clean_seat.chars() {
            match p {
                'F' => row_back = (row_back - ((row_back - row_front) / 2.0)).floor(),
                'B' => row_front = (row_front + ((row_back - row_front) / 2.0)).ceil(),
                'L' => col_right = (col_right - ((col_right - col_left) / 2.0)).floor(),
                'R' => col_left = (col_right - ((col_right - col_left) / 2.0)).ceil(),
                _ => return Err(err::ParseError::new("invalid seat format", seat)),
            };
            // println!("{}: {} - {}, {} - {}", p, row_front, row_back, col_left, col_right);
        }

        Ok(Seat::new(row_front as u32, col_left as u32))
    }

    fn seat_id(&self, seat: &Seat) -> u32 {
        seat.row() * self.cols + seat.col()
    }
}

#[cfg(test)]
mod tests {
    mod plane {
        use super::super::*;

        #[test]
        fn find_seat() {
            let p = Plane::new(128, 8);

            assert_eq!(p.find_seat("FBFBBFFRLR"), Ok(Seat { row: 44, col: 5 }),);
            assert_eq!(p.find_seat("BFFFBBFRRR"), Ok(Seat { row: 70, col: 7 }),);
            assert_eq!(p.find_seat("FFFBBBFRRR"), Ok(Seat { row: 14, col: 7 }),);
            assert_eq!(p.find_seat("BBFFBBFRLL"), Ok(Seat { row: 102, col: 4 }),);
        }

        #[test]
        fn seat_id() {
            let p = Plane::new(128, 8);

            assert_eq!(p.seat_id(&Seat::new(44, 5)), 357,)
        }
    }
}
