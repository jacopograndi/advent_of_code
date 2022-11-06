use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for OpCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Self::Acc),
            "nop" => Ok(Self::Nop),
            "jmp" => Ok(Self::Jmp),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Operation {
    opcode: OpCode,
    arg: i32,
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, arg) = s.split_once(" ").unwrap();
        Ok(Operation {
            opcode: opcode.trim().parse::<OpCode>().unwrap(),
            arg: arg.trim().parse().unwrap(),
        })
    }
}

#[derive(Default, Clone)]
struct Program {
    ops: Vec<Operation>,
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut program = Program::default();
        for line in s.split("\n") {
            if !line.is_empty() {
                let op = line.trim().parse::<Operation>().unwrap();
                program.ops.push(op);
            }
        }
        Ok(program)
    }
}

#[derive(Clone)]
struct Processor {
    program: Program,
    acc: i32,
    cursor: i32,
    visited: Vec<i32>,
}

impl Processor {
    fn run_until_loop(&mut self) -> i32 {
        loop {
            let current_instruction = &self.program.ops[self.cursor as usize];
            if self.visited.contains(&self.cursor) {
                break;
            }
            self.visited.push(self.cursor);
            match current_instruction.opcode {
                OpCode::Acc => {
                    self.cursor += 1;
                    self.acc += current_instruction.arg
                }
                OpCode::Jmp => self.cursor += current_instruction.arg,
                OpCode::Nop => self.cursor += 1,
            }
            if !(0..(self.program.ops.len() as i32 - 1)).contains(&self.cursor) {
                break;
            }
        }
        self.acc
    }
}

fn main() {
    let processor = Processor {
        program: fs::read_to_string("input.txt").unwrap().parse().unwrap(),
        acc: 0,
        cursor: 0,
        visited: vec![],
    };
    println!(
        "Value of the accumulator at loop detection: {}",
        processor.clone().run_until_loop()
    );

    for i in 0..processor.program.ops.len() {
        let mut attempt = processor.clone();
        match attempt.program.ops[i as usize].opcode {
            OpCode::Nop => attempt.program.ops[i as usize].opcode = OpCode::Jmp,
            OpCode::Jmp => attempt.program.ops[i as usize].opcode = OpCode::Nop,
            OpCode::Acc => (),
        }
        attempt.run_until_loop();
        if attempt.cursor == attempt.program.ops.len() as i32 - 1 {
            println!(
                "Value of the accumulator of the fixed program (by changing {}): {}",
                i,
                attempt.clone().run_until_loop(),
            );
            break;
        }
    }
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn simple_loop() {
        let input = "
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6";
        let processor = Processor {
            program: input.parse::<Program>().unwrap(),
            acc: 0,
            cursor: 0,
            visited: vec![],
        };
        assert_eq!(5, processor.clone().run_until_loop());
        for i in 0..processor.program.ops.len() {
            let mut attempt = processor.clone();
            match attempt.program.ops[i as usize].opcode {
                OpCode::Nop => attempt.program.ops[i as usize].opcode = OpCode::Jmp,
                OpCode::Jmp => attempt.program.ops[i as usize].opcode = OpCode::Nop,
                OpCode::Acc => (),
            }
            attempt.run_until_loop();
            if attempt.cursor == attempt.program.ops.len() as i32 {
                assert_eq!(8, attempt.clone().run_until_loop(),);
                break;
            }
        }
    }
}
