use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

use crate::days;
use crate::err;

#[derive(Debug)]
pub struct Day{}
impl days::Day for Day {
    fn run(&self) {
        println!("running day 2");
        let mut file = match File::open("data/02/input.txt") {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(e) => panic!(e),
            _ => {}
        };

        let pws = match parse_passwords::<TobogganValidator>(&contents) {
            Ok(pws) => pws,
            Err(e) => panic!("couldn't parse: {:?}", e),
        };

        let valid_count = pws
            .iter()
            .fold(0, |acc, pw| if pw.is_valid() { acc + 1 } else { acc });

        println!("valid pws: {}", valid_count);
    }
}

trait PasswordValidator: Sized {
    fn parse(s: &str) -> Result<Self, err::ParseError>;

    fn is_valid(&self, _pass: &str) -> bool {
        true
    }
}

#[derive(PartialEq, Debug)]
struct SledValidator {
    req_sequence: String,
    req_min: u32,
    req_max: u32,
}

impl SledValidator {
    fn seq_count(&self, pass: &str) -> u32 {
        pass.matches(&self.req_sequence)
            .collect::<Vec<&str>>()
            .len() as u32
    }
}

impl PasswordValidator for SledValidator {
    fn parse(s: &str) -> Result<Self, err::ParseError> {
        let re = match Regex::new(r"(?P<min>[0-9]+)-(?P<max>[0-9]+)\s+(?P<seq>[a-z]+)") {
            Ok(re) => re,
            Err(_) => return Err(err::ParseError::new("invalid regex", s)),
        };
        let caps = match re.captures(s) {
            Some(caps) => caps,
            _ => return Err(err::ParseError::new("invalid validator format", s)),
        };

        Ok(SledValidator {
            req_sequence: match caps.name("seq") {
                Some(v) => v.as_str().to_owned(),
                _ => return Err(err::ParseError::new("invalid validator format", s)),
            },
            req_min: match caps.name("min") {
                Some(v) => match v.as_str().parse::<u32>() {
                    Ok(v) => v,
                    _ => return Err(err::ParseError::new("invalid min value", s)),
                },
                _ => return Err(err::ParseError::new("invalid validator format", s)),
            },
            req_max: match caps.name("max") {
                Some(v) => match v.as_str().parse::<u32>() {
                    Ok(v) => v,
                    _ => return Err(err::ParseError::new("invalid max value", s)),
                },
                _ => return Err(err::ParseError::new("invalid validator format", s)),
            },
        })
    }
    fn is_valid(&self, pass: &str) -> bool {
        let count = self.seq_count(pass);
        count >= self.req_min && count <= self.req_max
    }
}

#[derive(PartialEq, Debug)]
struct TobogganValidator {
    req_sequence: String,
    position1: u32,
    position2: u32,
}

impl TobogganValidator {
    fn has_seq(&self, p: &str, start_pos: u32) -> bool {
        if start_pos == 0 { return false }

        let seq_len = self.req_sequence.len();
        let start_idx = (start_pos - 1) as usize;
        if start_idx + seq_len> p.len() {
            return false
        }

        for seq_idx in 0..seq_len {
            let idx = start_idx + seq_idx;
            if self.req_sequence.chars().nth(seq_idx) != p.chars().nth(idx) {
                return false
            }
        } 

        return true
    }
}

impl PasswordValidator for TobogganValidator {
    fn parse(s: &str) -> Result<Self, err::ParseError> {
        let re = match Regex::new(r"(?P<pos1>[0-9]+)-(?P<pos2>[0-9]+)\s+(?P<seq>[a-z]+)") {
            Ok(re) => re,
            Err(_) => return Err(err::ParseError::new("invalid regex", s)),
        };
        let caps = match re.captures(s) {
            Some(caps) => caps,
            _ => return Err(err::ParseError::new("invalid validator format", s)),
        };

        Ok(TobogganValidator {
            req_sequence: match caps.name("seq") {
                Some(v) => v.as_str().to_owned(),
                _ => return Err(err::ParseError::new("invalid validator format", s)),
            },
            position1: match caps.name("pos1") {
                Some(v) => match v.as_str().parse::<u32>() {
                    Ok(v) => v,
                    _ => return Err(err::ParseError::new("invalid min value", s)),
                },
                _ => return Err(err::ParseError::new("invalid validator format", s)),
            },
            position2: match caps.name("pos2") {
                Some(v) => match v.as_str().parse::<u32>() {
                    Ok(v) => v,
                    _ => return Err(err::ParseError::new("invalid max value", s)),
                },
                _ => return Err(err::ParseError::new("invalid validator format", s)),
            },
        })
    }
    fn is_valid(&self, pass: &str) -> bool {
        let mut matches = 0;
        if self.has_seq(pass, self.position1) {
            matches += 1;
        }
        if self.has_seq(pass, self.position2) {
            matches += 1;
        }

        match matches {
            1 => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Password<V> {
    validator: V,
    value: String,
}

fn parse_passwords<V: PasswordValidator>(s: &str) -> Result<Vec<Password<V>>, err::ParseError> {
    let lines = s.split("\n");

    let mut passwords = Vec::new();
    for line in lines {
        if line.trim() == "" {
            continue;
        }

        let pass = match Password::<V>::parse(line) {
            Ok(pass) => pass,
            Err(e) => return Err(e),
        };
        passwords.push(pass);
    }

    Ok(passwords)
}

impl<V: PasswordValidator> Password<V> {
    fn parse(s: &str) -> Result<Password<V>, err::ParseError> {
        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() != 2 {
            return Err(err::ParseError::new("invalid format", s));
        }

        Ok(Password {
            validator: match V::parse(parts[0]) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            value: parts[1].trim().to_owned(),
        })
    }

    fn is_valid(&self) -> bool {
        self.validator.is_valid(&self.value)
    }
}

#[cfg(test)]
mod tests {
    mod sledpass {
        use super::super::*;

        #[test]
        fn parse_valid() {
            assert_eq!(
                Password::<SledValidator>::parse("1-10 a: laskfdjlkasjd"),
                Ok(Password::<SledValidator> {
                    validator: SledValidator {
                        req_sequence: String::from("a"),
                        req_min: 1,
                        req_max: 10,
                    },
                    value: String::from("laskfdjlkasjd"),
                })
            )
        }
        #[test]
        fn parse_no_min() {
            assert_eq!(
                Password::<SledValidator>::parse("-10 a: laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "-10 a")),
            )
        }

        #[test]
        fn parse_no_max() {
            assert_eq!(
                Password::<SledValidator>::parse("1- a: laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "1- a")),
            )
        }

        #[test]
        fn parse_no_seq() {
            assert_eq!(
                Password::<SledValidator>::parse("1-10: laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "1-10")),
            );
            assert_eq!(
                Password::<SledValidator>::parse("1-10    : laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "1-10    ")),
            );
        }

        #[test]
        fn is_valid_valid() {
            let p = Password::<SledValidator>::parse("1-10 a: aabbcc").unwrap();
            assert_eq!(p.is_valid(), true);
        }

        #[test]
        fn is_valid_too_few() {
            let p = Password::<SledValidator>::parse("1-10 a: zzbbcc").unwrap();
            assert_eq!(p.is_valid(), false);
        }

        #[test]
        fn is_valid_too_many() {
            let p = Password::<SledValidator>::parse("1-3 a: aaaabbcc").unwrap();
            assert_eq!(p.is_valid(), false);
        }
    }
    
    mod tobogganpass {
        use super::super::*;

        #[test]
        fn parse_valid() {
            assert_eq!(
                Password::<TobogganValidator>::parse("1-10 a: laskfdjlkasjd"),
                Ok(Password::<TobogganValidator> {
                    validator: TobogganValidator {
                        req_sequence: String::from("a"),
                        position1: 1,
                        position2: 10,
                    },
                    value: String::from("laskfdjlkasjd"),
                })
            )
        }
        #[test]
        fn parse_no_min() {
            assert_eq!(
                Password::<TobogganValidator>::parse("-10 a: laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "-10 a")),
            )
        }

        #[test]
        fn parse_no_max() {
            assert_eq!(
                Password::<TobogganValidator>::parse("1- a: laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "1- a")),
            )
        }

        #[test]
        fn parse_no_seq() {
            assert_eq!(
                Password::<TobogganValidator>::parse("1-10: laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "1-10")),
            );
            assert_eq!(
                Password::<TobogganValidator>::parse("1-10    : laskfdjlkasjd"),
                Err(err::ParseError::new("invalid validator format", "1-10    ")),
            );
        }

        #[test]
        fn is_valid_first_valid() {
            let p = Password::<TobogganValidator>::parse("3-10 a: zbabzc").unwrap();
            assert_eq!(p.is_valid(), true);
        }

        #[test]
        fn is_valid_no_match() {
            let p = Password::<TobogganValidator>::parse("1-10 a: zbabzc").unwrap();
            assert_eq!(p.is_valid(), false);
        }

        #[test]
        fn is_valid_too_many_matches() {
            let p = Password::<TobogganValidator>::parse("1-5 a: ababaz").unwrap();
            assert_eq!(p.is_valid(), false);
        }
    }
}
