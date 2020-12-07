use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::days;
use crate::err;

#[derive(Debug)]
pub struct Day {}
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

        let groups = match parse_groups(&contents) {
            Ok(groups) => groups,
            Err(e) => {
                println!("could not parse groups: {:?}", e);
                return
            }
        };

        println!("yes questions: {}", groups.iter().fold(0, |acc, g| acc + g.yes_total()));
    }
}

struct Survey {
    yes_answers: Vec<char>,
}

impl Survey {
    fn new() -> Survey {
        Survey {
            yes_answers: Vec::new(),
        }
    }
    fn with_yes_answers(answers: Vec<char>) -> Survey {
        Survey{
            yes_answers: answers,
        }
    }
    fn parse(s: &str) -> Result<Survey, err::ParseError> {
        Ok(Survey{
            yes_answers: s.chars().filter(|c| *c != ' ').collect(),
        })
    }
}

fn parse_groups(s: &str) -> Result<Vec<Group>, err::ParseError> {
    let raw_groups: Vec<&str> = s.split("\n\n").map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

    let mut groups = Vec::new();
    for raw_group in raw_groups {
        groups.push(Group::parse(raw_group)?);
    }
    Ok(groups)
}

struct Group {
    surveys: Vec<Survey>,
}

impl Group {
    fn parse(s: &str) -> Result<Group, err::ParseError> {
        let lines: Vec<&str> = s
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let mut surveys = Vec::new();
        for line in lines {
            surveys.push(Survey::parse(line)?);
        }

        Ok(Group{
            surveys: surveys,
        })
    }

    fn member_count(&self) -> u32 {
        self.surveys.len() as u32
    }

    fn yes_total(&self) -> u32 {
        let mut yes_questions: HashMap<char, u32> = HashMap::new();
        for survey in &self.surveys {
            for question in &survey.yes_answers {
                yes_questions.insert(*question, match yes_questions.get(question) {
                    Some(total) => total + 1,
                    None => 1,
                });
            }
        }

        let total_members = self.member_count();
        yes_questions.iter().filter(|(_question, total)| **total == total_members).collect::<HashMap<&char, &u32>>().len() as u32
    }
}

#[cfg(test)]
mod tests {
    mod group {
        use super::super::*;

        #[test]
        fn yes_total() {
            let g = Group{
                surveys: vec![
                    Survey::with_yes_answers(vec!['a']),
                    Survey::with_yes_answers(vec!['b']),
                    Survey::with_yes_answers(vec!['c']),
                ],
            };
            assert_eq!(g.yes_total(), 3);
            
            let g = Group{
                surveys: vec![
                    Survey::with_yes_answers(vec!['a','b','c']),
                ],
            };
            assert_eq!(g.yes_total(), 3);
            
            let g = Group{
                surveys: vec![
                    Survey::with_yes_answers(vec!['a','b']),
                    Survey::with_yes_answers(vec!['a','c']),
                ],
            };
            assert_eq!(g.yes_total(), 3);
            
            let g = Group{
                surveys: vec![
                    Survey::with_yes_answers(vec!['a']),
                    Survey::with_yes_answers(vec!['a']),
                    Survey::with_yes_answers(vec!['a']),
                    Survey::with_yes_answers(vec!['a']),
                ],
            };
            assert_eq!(g.yes_total(), 1);
        }
    }
}