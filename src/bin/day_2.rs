use std::str::FromStr;

const INPUT: &str = include_str!("day_2_input");
const DESIRED_OUTPUT: u32 = 19_690_720;

fn main() {
    let memory: Vec<_> = INPUT
        .trim()
        .split(',')
        .map(|i| u32::from_str(i).unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = memory.clone();
            add_inputs(noun, verb, &mut memory);

            if DESIRED_OUTPUT == parse(&mut memory) {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn add_inputs(noun: u32, verb: u32, memory: &mut [u32]) {
    memory[1] = noun;
    memory[2] = verb;
}

fn parse(mut memory: &mut [u32]) -> u32 {
    let mut cursor = 0;

    while cursor < memory.len() - 1 {
        let params = (
            memory[cursor + 1] as usize,
            memory[cursor + 2] as usize,
            memory[cursor + 3] as usize,
        );

        let instruction = {
            match memory[cursor] {
                1 => Instruction::Add(params.0, params.1, params.2),
                2 => Instruction::Multiply(params.0, params.1, params.2),
                99 => Instruction::Halt,
                _ => {
                    // Ignore and skip to next instruction if the current instruction is not an opcode
                    cursor += 1;
                    continue;
                }
            }
        };

        if let Instruction::Halt = instruction {
            break;
        }

        instruction.exec(&mut memory);
        cursor += 4;
    }

    memory[0]
}

enum Instruction {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    Halt,
}

impl Instruction {
    fn exec(self, memory: &mut [u32]) {
        match self {
            Instruction::Add(a, b, out) => memory[out] = memory[a] + memory[b],
            Instruction::Multiply(a, b, out) => memory[out] = memory[a] * memory[b],
            _ => (),
        }
    }
}
