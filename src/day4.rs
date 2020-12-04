use crate::err;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = match File::open("data/04/input.txt") {
        Ok(file) => file,
        Err(e) => panic!(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!(e),
        _ => {}
    };

    let passports = match parse_passports(&contents) {
        Ok(p) => p,
        Err(e) => {
            println!("could not parse passports: {:?}", e);
            return
        }
    };

    println!(
        "valid passports: {}",
        passports.iter().filter(|p| p.is_valid()).count()
    );
}

#[derive(PartialEq, Debug)]
enum Height {
    In(u8),
    Cm(u8),
}

impl Height {
    fn parse(s: &str) -> Result<Height, err::ParseError> {
        let clean_s = s.trim().to_lowercase();
        if clean_s.len() < 3 {
            return Err(err::ParseError::new("invalid height format", s))
        }

        let num: String = clean_s.chars().take_while(|c| *c >= '0' && *c <= '9').collect();
        let unit: String = clean_s.chars().skip_while(|c| *c >= '0' && *c <= '9').collect();

        Ok(match num.parse::<u8>() {
            Ok(size) => match unit.as_str() {
                "cm" => Height::Cm(size),
                "in" => Height::In(size),
                _ => return Err(err::ParseError::new("invalid height format", s)),
            },
            Err(_) => return Err(err::ParseError::new("invalid height size", s)),
        })
    }
}

#[derive(PartialEq, Debug)]
enum PassportField {
    BirthYear(u16),
    IssueYear(u16),
    ExpirationYear(u16),
    Height(Height),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String),
}

impl PassportField {
    fn parse(s: &str) -> Result<PassportField, err::ParseError> {
        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() != 2 {
            return Err(err::ParseError::new("invalid field format", s));
        }

        let field_key = parts[0].trim();
        let field_value = parts[1].trim().to_owned();

        Ok(match field_key.to_lowercase().as_str() {
            "byr" => match field_value.parse::<u16>() {
                Ok(v) => PassportField::BirthYear(v),
                _ => return Err(err::ParseError::new("invalid format", s)),
            },
            "iyr" => match field_value.parse::<u16>() {
                Ok(v) => PassportField::IssueYear(v),
                _ => return Err(err::ParseError::new("invalid format", s)),
            },
            "eyr" => match field_value.parse::<u16>() {
                Ok(v) => PassportField::ExpirationYear(v),
                _ => return Err(err::ParseError::new("invalid format", s)),
            },
            "hgt" => match Height::parse(&field_value) {
                Ok(v) => PassportField::Height(v),
                _ => return Err(err::ParseError::new("invalid format", s)),
            },
            "hcl" => PassportField::HairColor(field_value),
            "ecl" => PassportField::EyeColor(field_value),
            "pid" => PassportField::PassportId(field_value),
            "cid" => PassportField::CountryId(field_value),
            _ => return Err(err::ParseError::new("unknown field", s)),
        })
    }

    // TODO move field validation here and just call that...
}

#[derive(PartialEq, Debug)]
struct Passport {
    fields: Vec<PassportField>,
}

impl Passport {
    fn parse(s: &str) -> Result<Passport, err::ParseError> {
        let raw_fields: Vec<&str> = s
            .split_whitespace()
            .map(|f| f.trim())
            .filter(|f| *f != "")
            .collect();

        let mut fields = Vec::new();
        for raw_field in raw_fields {
            match PassportField::parse(raw_field) {
                Ok(f) => fields.push(f),
                Err(e) => return Err(e),
            }
        }

        Ok(Passport { fields: fields })
    }

    fn is_valid(&self) -> bool {
        // TODO I don't like how this is handled but I don't have a better idea right now.
        let required_fields = vec![
            PassportField::BirthYear(0),
            PassportField::IssueYear(0),
            PassportField::ExpirationYear(0),
            PassportField::Height(Height::In(0)),
            PassportField::HairColor(String::new()),
            PassportField::EyeColor(String::new()),
            PassportField::PassportId(String::new()),
        ];

        for req_field in required_fields {
            let fields: Vec<&PassportField> = self
                .fields
                .iter()
                .filter(|f| std::mem::discriminant(*f) == std::mem::discriminant(&req_field))
                .map(|f| f)
                .collect();

            if fields.len() != 1 {
                return false;
            }

            // Additional field validation
            let hair_pattern = regex::Regex::new(r"#[a-f0-9]{6}").unwrap();
            match fields[0] {
                &PassportField::BirthYear(y) if y < 1920 || y > 2002 => return false,
                &PassportField::IssueYear(y) if y < 2010 || y > 2020 => return false,
                &PassportField::ExpirationYear(y) if y < 2020 || y > 2030 => return false,
                &PassportField::Height(Height::Cm(s)) if s < 150 || s > 193 => return false,
                &PassportField::Height(Height::In(s)) if s < 59 || s > 76 => return false,
                &PassportField::HairColor(ref c) if !hair_pattern.is_match(c) => return false,
                &PassportField::EyeColor(ref c) => match c.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
                    _ => return false,
                },
                &PassportField::PassportId(ref c) if c.len() != 9 || !c.parse::<u64>().is_ok() => return false,
                _ => {},
            }
        }

        true
    }
}

fn parse_passports(s: &str) -> Result<Vec<Passport>, err::ParseError> {
    let raw_passports: Vec<&str> = s
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| *p != "")
        .collect();

    let mut passports = Vec::new();
    for raw_passport in raw_passports {
        match Passport::parse(raw_passport) {
            Ok(p) => passports.push(p),
            // Throw out any passports that can't be parsed because they won't
            // be valid for our purposes anyway.
            Err(_) => {}
        }
    }

    Ok(passports)
}

#[cfg(test)]
mod tests {
    mod height {
        use super::super::*;

        #[test]
        fn parse_valid_inches() {
            assert_eq!(
                Height::parse("230in"),
                Ok(Height::In(230)),
            )
        }
        
        #[test]
        fn parse_valid_centimeters() {
            assert_eq!(
                Height::parse("230cm"),
                Ok(Height::Cm(230)),
            )
        }
    }

    mod passportfield {
        use super::super::*;

        #[test]
        fn parse_valid_birth_year() {
            assert_eq!(
                PassportField::parse("byr:1937"),
                Ok(PassportField::BirthYear(1937)),
            )
        }
        #[test]
        fn parse_valid_issue_year() {
            assert_eq!(
                PassportField::parse("iyr:1937"),
                Ok(PassportField::IssueYear(1937)),
            )
        }
        #[test]
        fn parse_valid_expiration_year() {
            assert_eq!(
                PassportField::parse("eyr:1937"),
                Ok(PassportField::ExpirationYear(1937)),
            )
        }
        #[test]
        fn parse_valid_height() {
            assert_eq!(
                PassportField::parse("hgt:183cm"),
                Ok(PassportField::Height(Height::Cm(183))),
            )
        }
        #[test]
        fn parse_valid_hair_color() {
            assert_eq!(
                PassportField::parse("hcl:#fffffd"),
                Ok(PassportField::HairColor(String::from("#fffffd"))),
            )
        }
        #[test]
        fn parse_valid_eye_color() {
            assert_eq!(
                PassportField::parse("ecl:gry"),
                Ok(PassportField::EyeColor(String::from("gry"))),
            )
        }
        #[test]
        fn parse_valid_passport_id() {
            assert_eq!(
                PassportField::parse("pid:860033327"),
                Ok(PassportField::PassportId(String::from("860033327"))),
            )
        }
        #[test]
        fn parse_valid_country_id() {
            assert_eq!(
                PassportField::parse("cid:350"),
                Ok(PassportField::CountryId(String::from("350"))),
            )
        }
    }

    mod passport {
        use super::super::*;

        #[test]
        fn parse_valid_syntax() {
            assert_eq!(
                Passport::parse(
                    r"
                    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                    byr:1937 iyr:2017 cid:147 hgt:183cm  
                    "
                ),
                Ok(Passport {
                    fields: vec![
                        PassportField::EyeColor(String::from("gry")),
                        PassportField::PassportId(String::from("860033327")),
                        PassportField::ExpirationYear(2020),
                        PassportField::HairColor(String::from("#fffffd")),
                        PassportField::BirthYear(1937),
                        PassportField::IssueYear(2017),
                        PassportField::CountryId(String::from("147")),
                        PassportField::Height(Height::Cm(183)),
                    ],
                })
            )
        }

        #[test]
        fn parse_multiple_valid_syntax() {
            assert_eq!(
                parse_passports(
                    r"
                    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                    byr:1937 iyr:2017 cid:147 hgt:183cm

                    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                    hcl:#cfa07d byr:1929

                    hcl:#ae17e1 iyr:2013
                    eyr:2024
                    ecl:brn pid:760753108 byr:1931
                    hgt:179cm

                    hcl:#cfa07d eyr:2025 pid:166559648
                    iyr:2011 ecl:brn hgt:59in
                    "
                ),
                Ok(vec![
                    Passport {
                        fields: vec![
                            PassportField::EyeColor(String::from("gry")),
                            PassportField::PassportId(String::from("860033327")),
                            PassportField::ExpirationYear(2020),
                            PassportField::HairColor(String::from("#fffffd")),
                            PassportField::BirthYear(1937),
                            PassportField::IssueYear(2017),
                            PassportField::CountryId(String::from("147")),
                            PassportField::Height(Height::Cm(183)),
                        ],
                    },
                    Passport {
                        fields: vec![
                            PassportField::IssueYear(2013),
                            PassportField::EyeColor(String::from("amb")),
                            PassportField::CountryId(String::from("350")),
                            PassportField::ExpirationYear(2023),
                            PassportField::PassportId(String::from("028048884")),
                            PassportField::HairColor(String::from("#cfa07d")),
                            PassportField::BirthYear(1929),
                        ],
                    },
                    Passport {
                        fields: vec![
                            PassportField::HairColor(String::from("#ae17e1")),
                            PassportField::IssueYear(2013),
                            PassportField::ExpirationYear(2024),
                            PassportField::EyeColor(String::from("brn")),
                            PassportField::PassportId(String::from("760753108")),
                            PassportField::BirthYear(1931),
                            PassportField::Height(Height::Cm(179)),
                        ],
                    },
                    Passport {
                        fields: vec![
                            PassportField::HairColor(String::from("#cfa07d")),
                            PassportField::ExpirationYear(2025),
                            PassportField::PassportId(String::from("166559648")),
                            PassportField::IssueYear(2011),
                            PassportField::EyeColor(String::from("brn")),
                            PassportField::Height(Height::In(59)),
                        ],
                    },
                ]),
            )
        }

        #[test]
        fn all_fields_is_valid() {
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:1937 iyr:2017 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
        }
        #[test]
        fn missing_field_is_not_valid() {
            match Passport::parse(
                r"
                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                hcl:#cfa07d byr:1929
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        #[test]
        fn missing_cid_is_valid() {
            match Passport::parse(
                r"
                hcl:#ae17e1 iyr:2013
                eyr:2024
                ecl:brn pid:760753108 byr:1931
                hgt:179cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
        }
        #[test]
        fn missing_multiple_fields_is_not_valid() {
            match Passport::parse(
                r"
                hcl:#cfa07d eyr:2025 pid:166559648
                iyr:2011 ecl:brn hgt:59in
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        #[test]
        fn not_valid_birth_year() {
            // Low
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:1919 iyr:2017 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
            // High
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:2003 iyr:2017 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        #[test]
        fn not_valid_issue_year() {
            // Low
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:1920 iyr:2009 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
            // High
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                byr:1920 iyr:2021 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        #[test]
        fn not_valid_expiration_year() {
            // Low
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2019 hcl:#fffffd
                byr:1920 iyr:2010 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
            // High
            match Passport::parse(
                r"
                ecl:gry pid:860033327 eyr:2031 hcl:#fffffd
                byr:1920 iyr:2010 cid:147 hgt:183cm
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        #[test]
        fn not_valid_height_cm() {
            // Low
            match Passport::parse(
                r"
                pid:087499704 hgt:149cm ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
            // High
            match Passport::parse(
                r"
                pid:087499704 hgt:194cm ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        #[test]
        fn not_valid_height_in() {
            // Low
            match Passport::parse(
                r"
                pid:087499704 hgt:58in ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
            // High
            match Passport::parse(
                r"
                pid:087499704 hgt:77in ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        // TODO test hair color matching

        #[test]
        fn valid_eye_color() {
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:amb iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:blu iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:brn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:gry iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:grn iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:hzl iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:oth iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), true),
                _ => panic!("invalid passport format"),
            }
        }

        #[test]
        fn not_valid_eye_color() {
            match Passport::parse(
                r"
                pid:087499704 hgt:59in ecl:kxd iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
        
        #[test]
        fn not_valid_passport_id() {
            match Passport::parse(
                r"
                pid:87499704 hgt:59in ecl:amb iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
            match Passport::parse(
                r"
                pid:a87499704 hgt:59in ecl:amb iyr:2012 eyr:2030 byr:1980
                hcl:#623a2f
                ",
            ) {
                Ok(p) => assert_eq!(p.is_valid(), false),
                _ => panic!("invalid passport format"),
            }
        }
    }
}
