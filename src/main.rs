use std::{
    env::args,
    fs::read_to_string,
    io::{self, Read},
};

#[derive(Debug)]
enum InstKind {
    Add(u8),
    Sub(u8),
    Move(i16),
    OutputByte,
    InputByte,
    JumpForward(u16),
    JumpBackward(u16),
}
impl InstKind {
    fn parse(s: &str) -> Option<(&str, Option<Self>)> {
        let first_char = s.chars().next()?;
        let mut count = 1;

        if matches!(first_char, '<' | '>' | '+' | '-') {
            for c in s.chars().skip(1) {
                if c == first_char {
                    count += 1;
                } else {
                    break;
                }
            }
        }
        //return Self::map_inst(first_char, count).map(|inst| (&s[..=count as usize], inst));
        Some((&s[..count], Self::map_inst(first_char, count as i16)))
    }

    fn map_inst(c: char, count: i16) -> Option<Self> {
        match c {
            '+' => Some(InstKind::Add(count as u8)),
            '-' => Some(InstKind::Sub(count as u8)),
            '>' => Some(InstKind::Move(count)),
            '<' => Some(InstKind::Move(-count)),
            '.' => Some(InstKind::OutputByte),
            ',' => Some(InstKind::InputByte),
            '[' => Some(InstKind::JumpForward(0)),
            ']' => Some(InstKind::JumpBackward(0)),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Interpreter {
    instructions: Vec<InstKind>,
    instruction_index: u16,
    data: Vec<u8>,
    pointer: i16,
}
impl Interpreter {
    fn new(input: &str) -> Self {
        let mut instructions = Vec::<InstKind>::new();

        let mut remaining = input;

        while let Some((sub_slice, inst)) = InstKind::parse(remaining) {
            if let Some(inst) = inst {
                instructions.push(inst);
            }
            remaining = &remaining[sub_slice.len()..]; // Update remaining to exclude the parsed part
        }
        let mut left_stack = Vec::new();
        for i in 0..instructions.len() {
            match instructions[i] {
                InstKind::JumpForward(_) => left_stack.push(i),
                InstKind::JumpBackward(_) => {
                    let opening = left_stack.pop().expect("unmatched ]");

                    instructions[opening] = InstKind::JumpForward(i as u16);
                    instructions[i] = InstKind::JumpBackward(opening as u16);
                }
                _ => (),
            }
        }
        assert!(left_stack.is_empty(), "unmatched [");
        Self {
            instruction_index: 0,
            instructions,
            data: vec![0; 30000],
            pointer: 0,
        }
    }
    fn next(&mut self) {
        match self.instructions[self.instruction_index as usize] {
            InstKind::Add(i) => {
                self.data[self.pointer as usize] =
                    self.data[self.pointer as usize].wrapping_add(i as u8)
            }
            InstKind::Sub(i) => {
                self.data[self.pointer as usize] =
                    self.data[self.pointer as usize].wrapping_sub(i as u8)
            }
            InstKind::Move(i) => self.pointer += i,
            InstKind::OutputByte => print!("{}", char::from(self.data[self.pointer as usize])),
            InstKind::JumpForward(new) => {
                if self.data[self.pointer as usize] == 0 {
                    self.instruction_index = new;
                }
            }
            InstKind::JumpBackward(new) => {
                if self.data[self.pointer as usize] != 0 {
                    self.instruction_index = new;
                }
            }
            InstKind::InputByte => {
                // does this even work?
                let mut buffer = [0];
                io::stdin()
                    .read_exact(&mut buffer)
                    .expect("failed to read input");
                self.data[self.pointer as usize] = buffer[0];
            }
        };
        self.instruction_index += 1;
    }
    fn run(&mut self) {
        while self.instruction_index != self.instructions.len() as u16 {
            self.next();
        }
    }
}

fn main() {
    let filename = args().nth(1).expect("usage: brainfuck file");
    let instructions = read_to_string(filename).expect("failed to read file");
    Interpreter::new(&instructions).run();
}
