use std::fs::File;
use std::io::prelude::*;

use crate::days;
use crate::err;

#[derive(Debug)]
pub struct Day {}
impl days::Day for Day {
    fn run(&self) {
        println!("running day 8");
        let mut file = match File::open("data/08/input.txt") {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(e) => panic!(e),
            _ => {}
        };

        let mut m = Machine::new();
        let prog = match Program::parse(&contents) {
            Ok(prog) => prog,
            Err(e) => panic!(e),
        };

        m.load(&prog);
        m.run();

        println!("acc: {}", m.acc());
    }
}

struct Machine<'a> {
    acc: i32,
    pc: u32,

    program: Option<&'a Program>,
    executed: Vec<u32>,
}

impl<'a> Machine<'a> {
    fn new() -> Machine<'a> {
        Machine {
            acc: 0,
            pc: 0,
            program: None,
            executed: Vec::new(),
        }
    }

    fn acc(&self) -> i32 {
        self.acc
    }

    fn load(&mut self, prog: &'a Program) {
        self.acc = 0;
        self.program = Some(prog);
    }

    fn run(&mut self) {
        while !self.step() {}
    }

    fn step(&mut self) -> bool {
        let ir = match &self.program {
            Some(p) => match p.ops().get(self.pc as usize) {
                Some(ir) => ir,
                None => return true,
            },
            None => return true,
        };

        if self.executed.contains(&self.pc) {
            return true
        }

        let old_pc = self.pc;
        match ir {
            Op::Nop => {
                self.pc += 1;
            }
            Op::Acc(acc) => {
                self.acc += acc;
                self.pc += 1;
            }
            Op::Jmp(jmp) => {
                self.pc = match self.pc as i32 + jmp {
                    v if v < 0 =>
                    /* TODO Invalid jmp, out of range */
                    {
                        return true
                    }
                    v => v as u32,
                };
            }
        }
        self.executed.push(old_pc);

        false
    }
}

#[derive(PartialEq, Debug)]
enum Op {
    Nop,
    Acc(i32),
    Jmp(i32),
}

impl Op {
    fn parse(s: &str) -> Result<Op, err::ParseError> {
        let parts: Vec<&str> = s
            .split_whitespace()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();
        if parts.len() != 2 {
            return Err(err::ParseError::new("invalid op format", s));
        }

        match parts[0] {
            "nop" => Ok(Op::Nop),
            "acc" => match parts[1].parse::<i32>() {
                Ok(amt) => Ok(Op::Acc(amt)),
                Err(e) => return Err(err::ParseError::new("invalid acc arg", s)),
            },
            "jmp" => match parts[1].parse::<i32>() {
                Ok(amt) => Ok(Op::Jmp(amt)),
                Err(e) => return Err(err::ParseError::new("invalid jmp arg", s)),
            },
            _ => return Err(err::ParseError::new("invalid op", s)),
        }
    }
}

struct Program {
    ops: Vec<Op>,
}

impl Program {
    fn parse(s: &str) -> Result<Program, err::ParseError> {
        let lines: Vec<&str> = s
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let mut ops = Vec::new();
        for line in lines {
            ops.push(Op::parse(line)?)
        }

        Ok(Program { ops: ops })
    }

    fn ops(&self) -> &Vec<Op> {
        &self.ops
    }
}

#[cfg(test)]
mod tests {
    mod op {
        use super::super::*;

        #[test]
        fn parse() {
            assert_eq!(Op::parse("acc +1"), Ok(Op::Acc(1)));
            assert_eq!(Op::parse("acc -1"), Ok(Op::Acc(-1)));
            assert_eq!(Op::parse("acc +10"), Ok(Op::Acc(10)));
            assert_eq!(Op::parse("acc -10"), Ok(Op::Acc(-10)));
            assert_eq!(Op::parse("jmp +1"), Ok(Op::Jmp(1)));
            assert_eq!(Op::parse("jmp -1"), Ok(Op::Jmp(-1)));
            assert_eq!(Op::parse("jmp +10"), Ok(Op::Jmp(10)));
            assert_eq!(Op::parse("jmp -10"), Ok(Op::Jmp(-10)));
            assert_eq!(Op::parse("nop +0"), Ok(Op::Nop));
        }
    }

    mod machine {
        use super::super::*;
        fn make_prog() -> Program {
            Program {
                ops: vec![
                    Op::Nop,
                    Op::Acc(1),
                    Op::Jmp(4),
                    Op::Acc(3),
                    Op::Jmp(-3),
                    Op::Acc(-99),
                    Op::Acc(1),
                    Op::Jmp(-4),
                    Op::Acc(6),
                ],
            }
        }

        #[test]
        fn step() {
            let mut m = Machine::new();
            let p = make_prog();
            m.load(&p);

            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 0);
            assert_eq!(m.pc, 1);
            assert_eq!(m.executed, vec![0]);
            
            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 1);
            assert_eq!(m.pc, 2);
            assert_eq!(m.executed, vec![0, 1]);
            
            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 1);
            assert_eq!(m.pc, 6);
            assert_eq!(m.executed, vec![0, 1, 2]);
            
            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 2);
            assert_eq!(m.pc, 7);
            assert_eq!(m.executed, vec![0, 1, 2, 6]);
            
            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 2);
            assert_eq!(m.pc, 3);
            assert_eq!(m.executed, vec![0, 1, 2, 6, 7]);
            
            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 5);
            assert_eq!(m.pc, 4);
            assert_eq!(m.executed, vec![0, 1, 2, 6, 7, 3]);
            
            assert_eq!(m.step(), false);
            assert_eq!(m.acc, 5);
            assert_eq!(m.pc, 1);
            assert_eq!(m.executed, vec![0, 1, 2, 6, 7, 3, 4]);
            
            assert_eq!(m.step(), true);
            assert_eq!(m.acc, 5);
            assert_eq!(m.pc, 1);
            assert_eq!(m.executed, vec![0, 1, 2, 6, 7, 3, 4]);
        }
    }
}
