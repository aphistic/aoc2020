use std::fs::File;
use std::io::prelude::*;

use crate::days;

#[derive(Debug)]
pub struct Day{}
impl days::Day for Day {
    fn run(&self) {
        println!("running day 6");
        let mut file = match File::open("data/06/input.txt") {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(e) => panic!(e),
            _ => {}
        };
    }
}
