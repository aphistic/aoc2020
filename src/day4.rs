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
        Err(e) => panic!(e),
    };

    println!("valid passports: {}", passports.iter().filter(|p| p.is_valid()).count());
}

#[derive(PartialEq, Debug)]
enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
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
            "byr" => PassportField::BirthYear(field_value),
            "iyr" => PassportField::IssueYear(field_value),
            "eyr" => PassportField::ExpirationYear(field_value),
            "hgt" => PassportField::Height(field_value),
            "hcl" => PassportField::HairColor(field_value),
            "ecl" => PassportField::EyeColor(field_value),
            "pid" => PassportField::PassportId(field_value),
            "cid" => PassportField::CountryId(field_value),
            _ => return Err(err::ParseError::new("unknown field", s)),
        })
    }
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
        let required_fields = vec![
            PassportField::BirthYear(String::new()),
            PassportField::IssueYear(String::new()),
            PassportField::ExpirationYear(String::new()),
            PassportField::Height(String::new()),
            PassportField::HairColor(String::new()),
            PassportField::EyeColor(String::new()),
            PassportField::PassportId(String::new()),
        ];

        for req_field in required_fields {
            let field_count = self
                .fields
                .iter()
                .filter(|f| std::mem::discriminant(*f) == std::mem::discriminant(&req_field))
                .count();

            if field_count < 1 {
                return false;
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
            Err(e) => return Err(e),
        }
    }

    Ok(passports)
}

#[cfg(test)]
mod tests {
    mod passportfield {
        use super::super::*;

        #[test]
        fn parse_valid_birth_year() {
            assert_eq!(
                PassportField::parse("byr:1937"),
                Ok(PassportField::BirthYear(String::from("1937"))),
            )
        }
        #[test]
        fn parse_valid_issue_year() {
            assert_eq!(
                PassportField::parse("iyr:1937"),
                Ok(PassportField::IssueYear(String::from("1937"))),
            )
        }
        #[test]
        fn parse_valid_expiration_year() {
            assert_eq!(
                PassportField::parse("eyr:1937"),
                Ok(PassportField::ExpirationYear(String::from("1937"))),
            )
        }
        #[test]
        fn parse_valid_height() {
            assert_eq!(
                PassportField::parse("hgt:183cm"),
                Ok(PassportField::Height(String::from("183cm"))),
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
                        PassportField::ExpirationYear(String::from("2020")),
                        PassportField::HairColor(String::from("#fffffd")),
                        PassportField::BirthYear(String::from("1937")),
                        PassportField::IssueYear(String::from("2017")),
                        PassportField::CountryId(String::from("147")),
                        PassportField::Height(String::from("183cm")),
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
                            PassportField::ExpirationYear(String::from("2020")),
                            PassportField::HairColor(String::from("#fffffd")),
                            PassportField::BirthYear(String::from("1937")),
                            PassportField::IssueYear(String::from("2017")),
                            PassportField::CountryId(String::from("147")),
                            PassportField::Height(String::from("183cm")),
                        ],
                    },
                    Passport {
                        fields: vec![
                            PassportField::IssueYear(String::from("2013")),
                            PassportField::EyeColor(String::from("amb")),
                            PassportField::CountryId(String::from("350")),
                            PassportField::ExpirationYear(String::from("2023")),
                            PassportField::PassportId(String::from("028048884")),
                            PassportField::HairColor(String::from("#cfa07d")),
                            PassportField::BirthYear(String::from("1929")),
                        ],
                    },
                    Passport {
                        fields: vec![
                            PassportField::HairColor(String::from("#ae17e1")),
                            PassportField::IssueYear(String::from("2013")),
                            PassportField::ExpirationYear(String::from("2024")),
                            PassportField::EyeColor(String::from("brn")),
                            PassportField::PassportId(String::from("760753108")),
                            PassportField::BirthYear(String::from("1931")),
                            PassportField::Height(String::from("179cm")),
                        ],
                    },
                    Passport {
                        fields: vec![
                            PassportField::HairColor(String::from("#cfa07d")),
                            PassportField::ExpirationYear(String::from("2025")),
                            PassportField::PassportId(String::from("166559648")),
                            PassportField::IssueYear(String::from("2011")),
                            PassportField::EyeColor(String::from("brn")),
                            PassportField::Height(String::from("59in")),
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
    }
}
