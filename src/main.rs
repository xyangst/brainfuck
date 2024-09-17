use std::{
    collections::BTreeMap,
    env::args,
    fs::read_to_string,
    io::{self, Read},
};

#[derive(Debug)]
enum InstKind {
    Add,
    Sub,
    PointerIncr,
    PointerDecr,
    OutputByte,
    InputByte,
    JumpForward,
    JumpBackward,
}
impl InstKind {
    fn parse(c: &char) -> Option<Self> {
        match c {
            '>' => Some(InstKind::PointerIncr),
            '<' => Some(InstKind::PointerDecr),
            '+' => Some(InstKind::Add),
            '-' => Some(InstKind::Sub),
            '.' => Some(InstKind::OutputByte),
            ',' => Some(InstKind::InputByte),
            '[' => Some(InstKind::JumpForward),
            ']' => Some(InstKind::JumpBackward),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Interpreter {
    instructions: Vec<InstKind>,
    instruction_index: usize,
    bracket_map: BTreeMap<usize, usize>,
    data: Vec<u8>,
    pointer: usize,
}
impl Interpreter {
    fn new(input: &str) -> Self {
        let mut instructions = Vec::new();
        let mut bracket_map: BTreeMap<usize, usize> = BTreeMap::new();
        let mut left_stack = Vec::new();
        for (i, thing) in input
            .chars()
            .filter_map(|c| InstKind::parse(&c))
            .enumerate()
        {
            match thing {
                InstKind::JumpForward => left_stack.push(i),
                InstKind::JumpBackward => {
                    let v = left_stack.pop().expect("unmatched ]");
                    bracket_map.insert(v, i);
                    bracket_map.insert(i, v);
                }
                _ => (),
            }
            instructions.push(thing)
        }
        assert!(left_stack.is_empty(), "unmatched [");
        Self {
            instruction_index: 0,
            bracket_map,
            instructions,
            data: vec![0; 30000],
            pointer: 0,
        }
    }
    fn next(&mut self) -> Option<()> {
        match self.instructions[self.instruction_index] {
            InstKind::Add => self.data[self.pointer] += 1,
            InstKind::Sub => self.data[self.pointer] -= 1,
            InstKind::PointerIncr => self.pointer += 1,
            InstKind::PointerDecr => self.pointer -= 1,
            InstKind::OutputByte => print!("{}", char::from(self.data[self.pointer])),
            InstKind::JumpForward => {
                if self.data[self.pointer] == 0 {
                    self.instruction_index =
                        *self.bracket_map.get(&self.instruction_index).unwrap();
                    return None;
                }
            }
            InstKind::JumpBackward => {
                if self.data[self.pointer] != 0 {
                    self.instruction_index =
                        *self.bracket_map.get(&self.instruction_index).unwrap();
                    return None;
                }
            }
            InstKind::InputByte => {
                let mut buffer = [0];
                io::stdin()
                    .read_exact(&mut buffer)
                    .expect("failed to read input");
                self.data[self.pointer] = buffer[0];
            }
        };
        Some(())
    }
    fn run(&mut self) {
        while self.instruction_index != self.instructions.len() {
            if self.next().is_some() {
                self.instruction_index += 1;
            }
        }
    }
}

fn main() {
    let filename = args().nth(1).expect("usage: brainfuck file");
    let instructions = read_to_string(filename).expect("failed to read file");
    Interpreter::new(&instructions).run();
}
