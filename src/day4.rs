use std::fs::File;
use std::io::prelude::*;

use crate::days;
use crate::err;

#[derive(Debug)]
pub struct Day{}
impl days::Day for Day {
    fn run(&self) {
        println!("running day 4");
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
                return;
            }
        };

        println!(
            "valid passports: {}",
            passports.iter().filter(|p| p.is_valid()).count()
        );
    }
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
            return Err(err::ParseError::new("invalid height format", s));
        }

        let num: String = clean_s
            .chars()
            .take_while(|c| *c >= '0' && *c <= '9')
            .collect();
        let unit: String = clean_s
            .chars()
            .skip_while(|c| *c >= '0' && *c <= '9')
            .collect();

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

    fn is_valid(&self) -> bool {
        // Additional field validation
        let hair_pattern = regex::Regex::new(r"#[a-f0-9]{6}").unwrap();
        match *self {
            PassportField::BirthYear(y) if y < 1920 || y > 2002 => false,
            PassportField::IssueYear(y) if y < 2010 || y > 2020 => false,
            PassportField::ExpirationYear(y) if y < 2020 || y > 2030 => false,
            PassportField::Height(Height::Cm(s)) if s < 150 || s > 193 => false,
            PassportField::Height(Height::In(s)) if s < 59 || s > 76 => false,
            PassportField::HairColor(ref c) if !hair_pattern.is_match(c) => false,
            PassportField::EyeColor(ref c) => match c.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => return false,
            },
            PassportField::PassportId(ref c) if c.len() != 9 || !c.parse::<u64>().is_ok() => false,
            _ => true,
        }
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

            if fields.len() != 1 || !fields[0].is_valid() {
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
            // Throw out any passports that can't be parsed because they won't
            // be valid for our purposes anyway.
            Err(_) => {}
        }
    }

    Ok(passports)
}

#[cfg(test)]
mod tests {
    macro_rules! validate_tests {
        ($($name:ident: $expected:tt, $value:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        assert_eq!($value.is_valid(), $expected);
                    }
                )*
        };
    }

    mod height {
        use super::super::*;

        #[test]
        fn parse_valid_inches() {
            assert_eq!(Height::parse("230in"), Ok(Height::In(230)),)
        }
        #[test]
        fn parse_valid_centimeters() {
            assert_eq!(Height::parse("230cm"), Ok(Height::Cm(230)),)
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

        validate_tests! {
            birth_year_below:  false, PassportField::BirthYear(1919),
            birth_year_bottom: true,  PassportField::BirthYear(1920),
            birth_year_mid:    true,  PassportField::BirthYear(2000),
            birth_year_top:    true,  PassportField::BirthYear(2002),
            birth_year_above:  false, PassportField::BirthYear(2003),
        }
        validate_tests! {
            issue_year_below:  false, PassportField::IssueYear(2009),
            issue_year_bottom: true,  PassportField::IssueYear(2010),
            issue_year_mid:    true,  PassportField::IssueYear(2015),
            issue_year_top:    true,  PassportField::IssueYear(2020),
            issue_year_above:  false, PassportField::IssueYear(2021),
        }
        validate_tests! {
            expiration_year_below:  false, PassportField::ExpirationYear(2019),
            expiration_year_bottom: true,  PassportField::ExpirationYear(2020),
            expiration_year_mid:    true,  PassportField::ExpirationYear(2025),
            expiration_year_top:    true,  PassportField::ExpirationYear(2030),
            expiration_year_above:  false, PassportField::ExpirationYear(2031),
        }
        validate_tests! {
            height_cm_below:  false, PassportField::Height(Height::Cm(149)),
            height_cm_bottom: true,  PassportField::Height(Height::Cm(150)),
            height_cm_mid:    true,  PassportField::Height(Height::Cm(160)),
            height_cm_top:    true,  PassportField::Height(Height::Cm(193)),
            height_cm_above:  false, PassportField::Height(Height::Cm(194)),
        }
        validate_tests! {
            height_in_below:  false, PassportField::Height(Height::In(58)),
            height_in_bottom: true,  PassportField::Height(Height::In(59)),
            height_in_mid:    true,  PassportField::Height(Height::In(65)),
            height_in_top:    true,  PassportField::Height(Height::In(76)),
            height_in_above:  false, PassportField::Height(Height::In(77)),
        }
        validate_tests! {
            hair_color_0: true, PassportField::HairColor(String::from("#000000")),
            hair_color_1: true, PassportField::HairColor(String::from("#111111")),
            hair_color_2: true, PassportField::HairColor(String::from("#222222")),
            hair_color_3: true, PassportField::HairColor(String::from("#333333")),
            hair_color_4: true, PassportField::HairColor(String::from("#444444")),
            hair_color_5: true, PassportField::HairColor(String::from("#555555")),
            hair_color_6: true, PassportField::HairColor(String::from("#666666")),
            hair_color_7: true, PassportField::HairColor(String::from("#777777")),
            hair_color_8: true, PassportField::HairColor(String::from("#888888")),
            hair_color_9: true, PassportField::HairColor(String::from("#999999")),
            hair_color_a: true, PassportField::HairColor(String::from("#aaaaaa")),
            hair_color_b: true, PassportField::HairColor(String::from("#bbbbbb")),
            hair_color_c: true, PassportField::HairColor(String::from("#cccccc")),
            hair_color_d: true, PassportField::HairColor(String::from("#dddddd")),
            hair_color_e: true, PassportField::HairColor(String::from("#eeeeee")),
            hair_color_f: true, PassportField::HairColor(String::from("#ffffff")),
            hair_color_low_seq:  true, PassportField::HairColor(String::from("#012345")),
            hair_color_mid_seq:  true, PassportField::HairColor(String::from("#6789ab")),
            hair_color_high_seq: true, PassportField::HairColor(String::from("#cdefed")),
            hair_color_invalid_with_hash: false, PassportField::HairColor(String::from("#kxdkxd")),
            hair_color_invalid_no_hash:   false, PassportField::HairColor(String::from("kxdkxd")),
            hair_color_valid_no_hash:     false, PassportField::HairColor(String::from("abcdef")),
        }
        validate_tests! {
            eye_color_amb: true, PassportField::EyeColor(String::from("amb")),
            eye_color_blu: true, PassportField::EyeColor(String::from("blu")),
            eye_color_brn: true, PassportField::EyeColor(String::from("brn")),
            eye_color_gry: true, PassportField::EyeColor(String::from("gry")),
            eye_color_grn: true, PassportField::EyeColor(String::from("grn")),
            eye_color_hzl: true, PassportField::EyeColor(String::from("hzl")),
            eye_color_oth: true, PassportField::EyeColor(String::from("oth")),
            eye_color_oth_caps:     false, PassportField::EyeColor(String::from("OTH")),
            eye_color_invalid:      false, PassportField::EyeColor(String::from("kxd")),
            eye_color_invalid_caps: false, PassportField::EyeColor(String::from("KXD")),
        }
        validate_tests! {
            passport_id_zeros: true, PassportField::PassportId(String::from("000000000")),
            passport_id_asc:   true, PassportField::PassportId(String::from("012345678")),
            passport_id_desc:  true, PassportField::PassportId(String::from("987654321")),
            passport_id_seq_zeros_start: true, PassportField::PassportId(String::from("000012345")),
            passport_id_a:           false, PassportField::PassportId(String::from("aaaaaaaaa")),
            passport_id_too_short:   false, PassportField::PassportId(String::from("12345")),
            passport_id_alpha_start: false, PassportField::PassportId(String::from("a12345678")),
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
            assert_eq!(
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
                }
                .is_valid(),
                true,
            );
        }
        #[test]
        fn missing_field_is_not_valid() {
            assert_eq!(
                Passport {
                    fields: vec![
                        PassportField::EyeColor(String::from("gry")),
                        PassportField::PassportId(String::from("860033327")),
                        PassportField::ExpirationYear(2020),
                        PassportField::HairColor(String::from("#fffffd")),
                        PassportField::BirthYear(1937),
                        PassportField::IssueYear(2017),
                        PassportField::CountryId(String::from("147")),
                        // Height missing
                    ],
                }
                .is_valid(),
                false,
            );
        }
        #[test]
        fn missing_cid_is_valid() {
            assert_eq!(
                Passport {
                    fields: vec![
                        PassportField::EyeColor(String::from("gry")),
                        PassportField::PassportId(String::from("860033327")),
                        PassportField::ExpirationYear(2020),
                        PassportField::HairColor(String::from("#fffffd")),
                        PassportField::BirthYear(1937),
                        PassportField::IssueYear(2017),
                        PassportField::Height(Height::Cm(183)),
                    ],
                }
                .is_valid(),
                true,
            );
        }
        #[test]
        fn missing_multiple_fields_is_not_valid() {
            assert_eq!(
                Passport {
                    fields: vec![
                        PassportField::EyeColor(String::from("gry")),
                        PassportField::PassportId(String::from("860033327")),
                        PassportField::ExpirationYear(2020),
                        PassportField::HairColor(String::from("#fffffd")),
                        PassportField::IssueYear(2017),
                        PassportField::Height(Height::Cm(183)),
                        // Missing country id (ok) and birth year (not ok)
                    ],
                }
                .is_valid(),
                false,
            );
        }
    }
}
