use std::fs::File;
use std::io::prelude::*;

use crate::days;
use crate::err;

#[derive(Debug)]
pub struct Day {}
impl days::Day for Day {
    fn run(&self) {
        println!("running day 7");
        let mut file = match File::open("data/07/input.txt") {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(e) => panic!(e),
            _ => {}
        };

        let rulebook = match Rulebook::parse(&contents) {
            Ok(rules) => rules,
            Err(e) => {
                println!("could not parse rules: {:?}", e);
                return;
            }
        };

        println!("style options: {}", rulebook.find_options(&ColorStyle::new("shiny", "gold")).len());
        println!("color options: {}", rulebook.color_options(&ColorStyle::new("shiny", "gold")).len());
        println!("bags inside: {}", rulebook.bags_inside(&ColorStyle::new("shiny", "gold")));
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct ColorStyle {
    style: String,
    color: String,
}

impl ColorStyle {
    fn new(style: &str, color: &str) -> ColorStyle {
        ColorStyle {
            style: style.to_owned(),
            color: color.to_owned(),
        }
    }
    fn parse(s: &str) -> Result<ColorStyle, err::ParseError> {
        let parts: Vec<&str> = s
            .split_whitespace()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();

        if parts.len() < 2 {
            return Err(err::ParseError::new("invalid format", s));
        }

        Ok(ColorStyle {
            style: parts[0].to_owned(),
            color: parts[1].to_owned(),
        })
    }
}

#[derive(PartialEq, Debug)]
struct Contents {
    amount: u32,
    style: ColorStyle,
}

impl Contents {
    fn parse(s: &str) -> Result<Option<Contents>, err::ParseError> {
        let parts: Vec<&str> = s
            .split_whitespace()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();
        if parts.len() < 2 {
            return Err(err::ParseError::new("invalid contents format", s));
        }

        match parts[0..2] {
            ["no", "other"] => Ok(None),
            _ => Ok(Some(Contents {
                amount: match parts[0] {
                    "no" => 0,
                    raw_amount => match raw_amount.parse() {
                        Ok(amount) => amount,
                        Err(_) => {
                            return Err(err::ParseError::new("invalid amount for contents", s))
                        }
                    },
                },
                style: ColorStyle::parse(&parts[1..].join(" "))?,
            })),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Rule {
    bag: ColorStyle,
    contents: Option<Vec<Contents>>,
}

impl Rule {
    fn parse(s: &str) -> Result<Rule, err::ParseError> {
        let main_parts: Vec<&str> = s
            .split(" contain ")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();
        if main_parts.len() != 2 {
            return Err(err::ParseError::new("invalid rule format", s));
        }

        let rule_bag = main_parts[0];
        let rule_constraints: Vec<&str> = main_parts[1]
            .split(",")
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .collect();

        let mut contents = Vec::new();
        for rule_contents in rule_constraints {
            match Contents::parse(rule_contents) {
                Ok(c) => match c {
                    Some(c) => contents.push(c),
                    None => {}
                },
                Err(e) => return Err(e),
            };
        }

        Ok(Rule {
            bag: ColorStyle::parse(rule_bag)?,
            contents: match contents.len() {
                0 => None,
                _ => Some(contents),
            },
        })
    }

    fn contains_style(&self, style: &ColorStyle) -> bool {
        match &self.contents {
            Some(contents) => !contents
                .iter()
                .filter(|c| c.style == *style)
                .collect::<Vec<&Contents>>()
                .is_empty(),
            None => false,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Rulebook {
    rules: Vec<Rule>,
}

impl Rulebook {
    fn parse(s: &str) -> Result<Rulebook, err::ParseError> {
        let lines: Vec<&str> = s
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let mut rules = Vec::new();
        for line in lines {
            match Rule::parse(line) {
                Ok(rule) => rules.push(rule),
                Err(e) => println!("parse err: {:?}", e),
            }
        }

        Ok(Rulebook { rules: rules })
    }
    
    fn find_rule(&self, style: &ColorStyle) -> Option<&Rule> {
        for rule in &self.rules {
            if rule.bag == *style {
                return Some(&rule)
            }
        }

        None
    }

    fn find_options(&self, style: &ColorStyle) -> Vec<&ColorStyle> {
        let mut found = Vec::new();
        for rule in &self.rules {
            if rule.contains_style(style) {
                found.push(&rule.bag)
            }
        }

        if found.len() > 0 {
            let mut next_depth = Vec::new();
            for found_style in &found {
                let mut next_found = self.find_options(found_style);
                next_depth.append(&mut next_found)
            }
            found.append(&mut next_depth);
        }

        found.sort();
        found.dedup();

        found
    }

    fn color_options(&self, style: &ColorStyle) -> Vec<&str> {
        let opts = self.find_options(style);

        let mut colors: Vec<&str> = opts.iter().map(|o| o.color.as_str()).collect();
        colors.sort();
        colors.dedup();

        colors
    }

    fn bags_inside(&self, style: &ColorStyle) -> u32 {
        let rule = match self.find_rule(style) {
            Some(rule) => rule,
            None => return 0,
        };

        let mut total = 0;
        if let Some(rule_contents) = &rule.contents {
            for check_style in rule_contents {
                // Add the number of bags for this style, plus
                // we need to multiply that by the number of the
                // bags INSIDE those bags.
                total += check_style.amount;
                total += check_style.amount * self.bags_inside(&check_style.style);
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    macro_rules! match_parse {
        ($($name:ident: $cmd:expr, $expect:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($cmd, Ok($expect));
                }
            )*
        };
    }

    mod color_style {
        use super::super::*;

        match_parse! {
            parse_light_red:    ColorStyle::parse("light red"), ColorStyle::new("light", "red"),
            parse_dark_yellow:  ColorStyle::parse("dark yellow"), ColorStyle::new("dark", "yellow"),
            parse_bright_plum:  ColorStyle::parse("bright plum"), ColorStyle::new("bright", "plum"),
            parse_muted_black:  ColorStyle::parse("muted black"), ColorStyle::new("muted", "black"),
            parse_shiny_olive:  ColorStyle::parse("shiny olive"), ColorStyle::new("shiny", "olive"),
            parse_vibrant_plum: ColorStyle::parse("vibrant plum"), ColorStyle::new("vibrant", "plum"),
            parse_faded_blue:   ColorStyle::parse("faded blue"), ColorStyle::new("faded", "blue"),
            parse_dotted_red:   ColorStyle::parse("dotted red"), ColorStyle::new("dotted", "red"),
            parse_posh_black:   ColorStyle::parse("posh black"), ColorStyle::new("posh", "black"),
            parse_wavy_gold:    ColorStyle::parse("wavy gold"), ColorStyle::new("wavy", "gold"),
            parse_pale_cyan:    ColorStyle::parse("pale cyan"), ColorStyle::new("pale", "cyan"),
            parse_dull_tomato:  ColorStyle::parse("dull tomato"), ColorStyle::new("dull", "tomato"),
            parse_plaid_aqua:   ColorStyle::parse("plaid aqua"), ColorStyle::new("plaid", "aqua"),
            parse_drab_aqua:    ColorStyle::parse("drab aqua"), ColorStyle::new("drab", "aqua"),
        }
    }
    mod contents {
        use super::super::*;

        match_parse! {
            parse_no_other:       Contents::parse("no other bags"), None,
            parse_7_light_red:    Contents::parse("7 light red"), Some(Contents{amount: 7, style: ColorStyle::new("light", "red")}),
            parse_10_muted_black: Contents::parse("10 muted black"), Some(Contents{amount: 10, style: ColorStyle::new("muted", "black")}),
        }
    }
    mod rule {
        use super::super::*;

        match_parse! {
            parse_rule_1:
                Rule::parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
                Rule{
                    bag: ColorStyle::new("light", "red"),
                    contents: Some(vec![
                        Contents{amount: 1, style: ColorStyle::new("bright", "white")},
                        Contents{amount: 2, style: ColorStyle::new("muted", "yellow")},
                    ]),
                },
            parse_rule_2:
                Rule::parse("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
                Rule{
                    bag: ColorStyle::new("vibrant", "plum"),
                    contents: Some(vec![
                        Contents{amount: 5, style: ColorStyle::new("faded", "blue")},
                        Contents{amount: 6, style: ColorStyle::new("dotted", "black")},
                    ]),
                },
            parse_rule_no_other_1:
                Rule::parse("faded blue bags contain no other bags."),
                Rule{
                    bag: ColorStyle::new("faded", "blue"),
                    contents: None,
                },
        }

        #[test]
        fn contains_style_included() {
            assert_eq!(
                Rule {
                    bag: ColorStyle::new("light", "red"),
                    contents: Some(vec![
                        Contents {
                            amount: 1,
                            style: ColorStyle::new("bright", "white"),
                        },
                        Contents {
                            amount: 2,
                            style: ColorStyle::new("muted", "yellow"),
                        },
                    ]),
                }
                .contains_style(&ColorStyle::new("muted", "yellow")),
                true,
            );
            assert_eq!(
                Rule {
                    bag: ColorStyle::new("light", "red"),
                    contents: Some(vec![
                        Contents {
                            amount: 1,
                            style: ColorStyle::new("bright", "white"),
                        },
                        Contents {
                            amount: 2,
                            style: ColorStyle::new("muted", "yellow"),
                        },
                    ]),
                }
                .contains_style(&ColorStyle::new("hyper", "black")),
                false,
            );
        }
        #[test]
        fn contains_style_no_contents() {
            assert_eq!(
                Rule {
                    bag: ColorStyle::new("light", "red"),
                    contents: None,
                }
                .contains_style(&ColorStyle::new("hyper", "black")),
                false,
            );
        }
    }

    mod rulebook {
        use super::super::*;

        fn make_options_book() -> Rulebook {
            Rulebook::parse(r"
                light red bags contain 1 bright white bag, 2 muted yellow bags.
                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                bright white bags contain 1 shiny gold bag.
                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                faded blue bags contain no other bags.
                dotted black bags contain no other bags.
            ").unwrap()
        }
        fn make_depth_book() -> Rulebook {
            Rulebook::parse(r"
                shiny gold bags contain 2 dark red bags.
                dark red bags contain 2 dark orange bags.
                dark orange bags contain 2 dark yellow bags.
                dark yellow bags contain 2 dark green bags.
                dark green bags contain 2 dark blue bags.
                dark blue bags contain 2 dark violet bags.
                dark violet bags contain no other bags.
            ").unwrap()
        }

        match_parse! {
            parse_book_1:
                Rulebook::parse(r"
                    light red bags contain 1 bright white bag, 2 muted yellow bags.
                    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                    bright white bags contain 1 shiny gold bag.
                    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                    dotted black bags contain no other bags.
                "),
                Rulebook {
                    rules: vec![
                        Rule{
                            bag: ColorStyle::new("light", "red"),
                            contents: Some(vec![
                                Contents{
                                    amount: 1,
                                    style: ColorStyle::new("bright", "white"),
                                },
                                Contents{
                                    amount: 2,
                                    style: ColorStyle::new("muted", "yellow"),
                                },
                            ]),
                        },
                        Rule{
                            bag: ColorStyle::new("dark", "orange"),
                            contents: Some(vec![
                                Contents{
                                    amount: 3,
                                    style: ColorStyle::new("bright", "white"),
                                },
                                Contents{
                                    amount: 4,
                                    style: ColorStyle::new("muted", "yellow"),
                                },
                            ]),
                        },
                        Rule{
                            bag: ColorStyle::new("bright", "white"),
                            contents: Some(vec![
                                Contents{
                                    amount: 1,
                                    style: ColorStyle::new("shiny", "gold"),
                                },
                            ]),
                        },
                        Rule{
                            bag: ColorStyle::new("dark", "olive"),
                            contents: Some(vec![
                                Contents{
                                    amount: 3,
                                    style: ColorStyle::new("faded", "blue"),
                                },
                                Contents{
                                    amount: 4,
                                    style: ColorStyle::new("dotted", "black"),
                                },
                            ]),
                        },
                        Rule{
                            bag: ColorStyle::new("dotted", "black"),
                            contents: None,
                        },

                    ],
                },
        }

        #[test]
        fn find_options_1() {
            let book = make_options_book();
            assert_eq!(
                book.find_options(&ColorStyle::new("muted", "yellow")),
                vec![
                    &ColorStyle::new("dark", "orange"),
                    &ColorStyle::new("light", "red"),
                ],
            )
        }
        
        #[test]
        fn find_options_2() {
            let book = make_options_book();
            assert_eq!(
                book.find_options(&ColorStyle::new("shiny", "gold")),
                vec![
                    &ColorStyle::new("bright", "white"),
                    &ColorStyle::new("dark", "orange"),
                    &ColorStyle::new("light", "red"),
                    &ColorStyle::new("muted", "yellow"),
                ],
            )
        }
        
        #[test]
        fn color_options_1() {
            let book = make_options_book();
            assert_eq!(
                book.color_options(&ColorStyle::new("shiny", "gold")),
                vec!["orange", "red", "white", "yellow"],
            )
        }
        
        #[test]
        fn color_options_2() {
            let book = make_options_book();
            assert_eq!(
                book.color_options(&ColorStyle::new("dotted", "black")),
                vec!["gold", "olive", "orange", "plum", "red", "white", "yellow"],
            )
        }
        
        #[test]
        fn contains_dark_violet() {
            let book = make_depth_book();
            assert_eq!(
                book.bags_inside(&ColorStyle::new("dark", "violet")),
                0,
            )
        }
        
        #[test]
        fn contains_dark_blue() {
            let book = make_depth_book();
            assert_eq!(
                book.bags_inside(&ColorStyle::new("dark", "blue")),
                2,
            )
        }
        
        #[test]
        fn contains_dark_green() {
            let book = make_depth_book();
            assert_eq!(
                book.bags_inside(&ColorStyle::new("dark", "green")),
                6,
            )
        }

        #[test]
        fn contains_shiny_gold() {
            let book = make_depth_book();
            assert_eq!(
                book.bags_inside(&ColorStyle::new("shiny", "gold")),
                126,
            )
        }
    }
}
