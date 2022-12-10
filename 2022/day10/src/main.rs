use std::fs;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else {
            if let Some((_, imm)) = s.split_once(" ") {
                let v = imm.parse().unwrap();
                Ok(Self::Addx(v))
            } else {
                Err(format!("unsupported op: {s}"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Program {
    instructions: Vec<Operation>,
}

impl FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Program {
            instructions: s
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|l| l.trim().parse().unwrap())
                .collect(),
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Cpu {
    x: i32,
    clock: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, clock: 0 }
    }

    fn run<T: Component>(&mut self, prog: &Program, comp: &mut T) {
        let mut iter = prog.instructions.iter();
        while let Some(next) = iter.next() {
            let (addx, duration) = match next {
                Operation::Noop => (0, 1),
                Operation::Addx(v) => (*v, 2),
            };
            for _ in 0..duration {
                self.clock += 1;
                comp.step(self);
            }
            self.x += addx;
        }
    }
}

trait Component {
    fn step(&mut self, cpu: &Cpu);
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct SignalMeter {
    strengths: Vec<Cpu>,
}

impl Component for SignalMeter {
    fn step(&mut self, cpu: &Cpu) {
        if cpu.clock % 40 == 20 {
            self.strengths.push(cpu.clone());
        }
    }
}

impl SignalMeter {
    fn score(&self) -> i32 {
        self.strengths.iter().map(|cpu| cpu.x * cpu.clock).sum()
    }
}

const DISPLAY_WIDTH: i32 = 40;
const DISPLAY_HEIGHT: i32 = 6;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Display {
    pixels: Vec<bool>,
}

impl Component for Display {
    fn step(&mut self, cpu: &Cpu) {
        let pix = i32::abs((cpu.clock - 1) % DISPLAY_WIDTH - cpu.x) <= 1;
        self.pixels.push(pix);
    }
}

impl ToString for Display {
    fn to_string(&self) -> String {
        let mut s = String::default();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let i = x + y * DISPLAY_WIDTH;
                s += if self.pixels[i as usize] { "#" } else { "." }
            }
            s += "\n";
        }
        s
    }
}

fn main() {
    let prog: Program = fs::read_to_string("input.txt").unwrap().parse().unwrap();
    let mut cpu = Cpu::new();
    let mut meter = SignalMeter::default();
    cpu.run(&prog, &mut meter);
    println!("signal score: {}", meter.score());

    let mut cpu = Cpu::new();
    let mut display = Display::default();
    cpu.run(&prog, &mut display);
    println!("display:\n{}", display.to_string());
}

#[cfg(test)]
mod test {
    use crate::*;

    const SIMPLE_ASM: &str = "noop\naddx 3\naddx -5";

    #[test]
    fn parse_simple() {
        let prog: Program = SIMPLE_ASM.parse().unwrap();
        assert_eq!(prog.instructions.len(), 3);
    }

    #[test]
    fn example() {
        let prog: Program = fs::read_to_string("test.txt").unwrap().parse().unwrap();
        let mut cpu = Cpu::new();
        let mut meter = SignalMeter::default();
        cpu.run(&prog, &mut meter);
        assert_eq!(meter.score(), 13140);
    }

    #[test]
    fn example_display() {
        let prog: Program = fs::read_to_string("test.txt").unwrap().parse().unwrap();
        let mut cpu = Cpu::new();
        let mut display = Display::default();
        cpu.run(&prog, &mut display);
        let result = fs::read_to_string("test_result.txt").unwrap();
        println!("{}", display.to_string());
        assert_eq!(display.to_string(), result);
    }
}
