use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = match File::open("data/02/input.txt") {
        Ok(file) => file,
        Err(e) => panic!(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!(e),
        _ => {}
    };

    let pws = match parse_passwords(&contents) {
        Ok(pws) => pws,
        Err(e) => panic!("couldn't parse: {}", e),
    };

    let valid_count = pws
        .iter()
        .fold(0, |acc, pw| if pw.is_valid() { acc + 1 } else { acc });

    println!("valid pws: {}", valid_count);
}

#[derive(PartialEq, Debug)]
struct Password {
    req_sequence: String,
    req_min: u32,
    req_max: u32,

    value: String,
}

fn parse_passwords(s: &str) -> Result<Vec<Password>, String> {
    let lines = s.split("\n");

    let mut passwords = Vec::new();
    for line in lines {
        if line.trim() == "" {
            continue;
        }

        let pass = match Password::parse(line) {
            Ok(pass) => pass,
            Err(e) => return Err(e),
        };
        passwords.push(pass);
    }

    Ok(passwords)
}

impl Password {
    fn parse(s: &str) -> Result<Password, String> {
        let re = match Regex::new(
            r"(?P<min>[0-9]+)-(?P<max>[0-9]+)\s+(?P<seq>[a-z]+):\s+(?P<pass>[a-z]+)",
        ) {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()),
        };

        let caps = match re.captures(s) {
            Some(caps) => caps,
            None => return Err(format!("invalid format: {}", s)),
        };

        let req_min = match caps.name("min") {
            Some(v) => match v.as_str().parse::<u32>() {
                Ok(v) => v,
                _ => return Err(String::from("min is not a u32")),
            },
            None => return Err(format!("invalid format: {}", s)),
        };
        let req_max = match caps.name("max") {
            Some(v) => match v.as_str().parse::<u32>() {
                Ok(v) => v,
                _ => return Err(String::from("max is not a u32")),
            },
            None => return Err(format!("invalid format: {}", s)),
        };
        let req_seq = match caps.name("seq") {
            Some(v) => v.as_str(),
            None => return Err(format!("invalid format: {}", s)),
        };

        let pass = match caps.name("pass") {
            Some(v) => v.as_str(),
            None => return Err(format!("invalid format: {}", s)),
        };

        Ok(Password {
            req_sequence: String::from(req_seq),
            req_min: req_min,
            req_max: req_max,

            value: String::from(pass),
        })
    }

    fn seq_count(&self) -> u32 {
        self.value
            .matches(&self.req_sequence)
            .collect::<Vec<&str>>()
            .len() as u32
    }

    fn is_valid(&self) -> bool {
        let count = self.seq_count();
        count >= self.req_min && count <= self.req_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        assert_eq!(
            Password::parse("1-10 a: laskfdjlkasjd"),
            Ok(Password {
                req_sequence: String::from("a"),
                req_min: 1,
                req_max: 10,

                value: String::from("laskfdjlkasjd"),
            })
        )
    }

    #[test]
    fn parse_no_min() {
        assert_eq!(
            Password::parse("-10 a: laskfdjlkasjd"),
            Err(String::from("invalid format")),
        )
    }
    #[test]
    fn parse_no_max() {
        assert_eq!(
            Password::parse("1- a: laskfdjlkasjd"),
            Err(String::from("invalid format")),
        )
    }
    #[test]
    fn parse_no_seq() {
        assert_eq!(
            Password::parse("1-10: laskfdjlkasjd"),
            Err(String::from("invalid format")),
        );
        assert_eq!(
            Password::parse("1-10    : laskfdjlkasjd"),
            Err(String::from("invalid format")),
        );
    }

    #[test]
    fn seq_count_one_char() {
        let p = Password::parse("1-10 a: aabbcc").unwrap();
        assert_eq!(p.seq_count(), 2);
    }
    #[test]
    fn seq_count_no_match() {
        let p = Password::parse("1-10 a: zzbbcc").unwrap();
        assert_eq!(p.seq_count(), 0);
    }
    #[test]
    fn seq_count_multi_char() {
        let p = Password::parse("1-10 ab: abbbcc").unwrap();
        assert_eq!(p.seq_count(), 1);
    }
    #[test]
    fn is_valid_valid() {
        let p = Password::parse("1-10 a: aabbcc").unwrap();
        assert_eq!(p.is_valid(), true);
    }
    #[test]
    fn is_valid_too_few() {
        let p = Password::parse("1-10 a: zzbbcc").unwrap();
        assert_eq!(p.is_valid(), false);
    }
    #[test]
    fn is_valid_too_many() {
        let p = Password::parse("1-3 a: aaaabbcc").unwrap();
        assert_eq!(p.is_valid(), false);
    }
}
