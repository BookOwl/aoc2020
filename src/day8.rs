use crate::Part;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop(i64),
    Acc(i64),
    Jump(i64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ExecutionResult {
    Loop(i64),
    Terminated(i64),
}

struct VM {
    instructions: Vec<Instruction>,
    pc: i64,
    acc: i64,
    seen: HashSet<i64>,
}

impl VM {
    fn parse_program(program: &str) -> Vec<Instruction> {
        let instruction_re = Regex::new(r"(?P<op>nop|acc|jmp) (?P<arg>[+\-]\d+)").unwrap();
        instruction_re.captures_iter(program).map(|cap| {
            let (op, arg) = (&cap["op"], cap["arg"].parse::<i64>().unwrap());
            match op {
                "nop" => Instruction::Nop(arg),
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jump(arg),
                _ => unreachable!("{} is not a valid instruction", op),
            }
        }).collect()
    }
    fn new_from_instructions(instructions: Vec<Instruction>) -> Self {
        VM {
            instructions,
            pc: 0,
            acc: 0,
            seen: HashSet::new(),
        }
    }

    fn step(&mut self) -> Option<ExecutionResult> {
        if self.seen.contains(&self.pc) {
            return Some(ExecutionResult::Loop(self.acc))
        }
        if self.pc >= self.instructions.len() as i64 {
            return Some(ExecutionResult::Terminated(self.acc))
        }
        self.seen.insert(self.pc);
        let op = self.instructions[self.pc as usize];
        match op {
            Instruction::Nop(_) => {},
            Instruction::Acc(arg) => self.acc += arg,
            Instruction::Jump(offset) => self.pc += (offset - 1),
        };
        self.pc += 1;
        None
    }
    fn run(&mut self) -> ExecutionResult {
        loop {
            match self.step() {
                Some(res) => return res,
                None => {},
            }
        }
    }
}

pub fn run(part: Part, input: &str) {
    match part {
        Part::First => part1(&input),
        Part::Second => part2(&input),
        Part::All => {
            part1(&input);
            part2(&input);
        },
    }
}

fn part1(input: &str) {
    let mut vm = VM::new_from_instructions(VM::parse_program(input));
    println!("{:?}", vm.run());
}

fn part2(input: &str) {
    let corrupted = VM::parse_program(input);
    for (i, op) in corrupted.iter().enumerate() {
        match op {
            Instruction::Acc(_) => continue,
            Instruction::Nop(arg) => {
                let mut instructions = corrupted.clone();
                instructions[i] = Instruction::Jump(*arg);
                let mut vm = VM::new_from_instructions(instructions);
                match vm.run() {
                    ExecutionResult::Loop(_) => continue,
                    ExecutionResult::Terminated(res) => {
                        println!("terminated with acc = {} after changing instruction {} from NOP {} to JMP {}", res, i, arg, arg);
                        return
                    }
                }
            },
            Instruction::Jump(arg) => {
                let mut instructions = corrupted.clone();
                instructions[i] = Instruction::Nop(*arg);
                let mut vm = VM::new_from_instructions(instructions);
                match vm.run() {
                    ExecutionResult::Loop(_) => continue,
                    ExecutionResult::Terminated(res) => {
                        println!("terminated with acc = {} after changing instruction {} from JMP {} to NOP {}", res, i, arg, arg);
                        return
                    }
                }
            },
        }
    };
    panic!("could not find a solution!!!")
}
