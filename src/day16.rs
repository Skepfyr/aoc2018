use std::{convert::TryInto, str::FromStr};

use bitvec::prelude::*;
use enum_iterator::{all, Sequence};
use eyre::{bail, eyre, Context, Result};
use tracing::instrument;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    input: include_str!("../input/day16.txt"),
    part1,
    part2,
};

#[derive(Debug, Clone, Copy, Sequence)]
#[repr(u8)]
enum OpCode {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: OpCode,
    input_a: u16,
    input_b: u16,
    output_c: u16,
}

#[derive(Debug, Clone, Copy)]
struct UnknownInstruction {
    opcode: u8,
    input_a: u16,
    input_b: u16,
    output_c: u16,
}

impl FromStr for UnknownInstruction {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split_ascii_whitespace();
        let opcode = input
            .next()
            .ok_or_else(|| eyre!("Expected opcode in instruction {}", s))?
            .parse()?;
        let input_a = input
            .next()
            .ok_or_else(|| eyre!("Expected input A in instruction {}", s))?
            .parse()?;
        let input_b = input
            .next()
            .ok_or_else(|| eyre!("Expected input B in instruction {}", s))?
            .parse()?;
        let output_c = input
            .next()
            .ok_or_else(|| eyre!("Expected output C in instruction {}", s))?
            .parse()?;
        Ok(Self {
            opcode,
            input_a,
            input_b,
            output_c,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Cpu {
    registers: [u16; 4],
}

impl Cpu {
    fn execute(
        mut self,
        Instruction {
            opcode,
            input_a: a,
            input_b: b,
            output_c: c,
        }: Instruction,
    ) -> Self {
        self.registers[c as usize] = match opcode {
            OpCode::AddR => self.registers[a as usize] + self.registers[b as usize],
            OpCode::AddI => self.registers[a as usize] + b,
            OpCode::MulR => self.registers[a as usize] * self.registers[b as usize],
            OpCode::MulI => self.registers[a as usize] * b,
            OpCode::BanR => self.registers[a as usize] & self.registers[b as usize],
            OpCode::BanI => self.registers[a as usize] & b,
            OpCode::BorR => self.registers[a as usize] | self.registers[b as usize],
            OpCode::BorI => self.registers[a as usize] | b,
            OpCode::SetR => self.registers[a as usize],
            OpCode::SetI => a,
            OpCode::GtIR => u16::from(a > self.registers[b as usize]),
            OpCode::GtRI => u16::from(self.registers[a as usize] > b),
            OpCode::GtRR => u16::from(self.registers[a as usize] > self.registers[b as usize]),
            OpCode::EqIR => u16::from(a == self.registers[b as usize]),
            OpCode::EqRI => u16::from(self.registers[a as usize] == b),
            OpCode::EqRR => u16::from(self.registers[a as usize] == self.registers[b as usize]),
        };
        self
    }
}

impl FromStr for Cpu {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let registers = s
            .strip_prefix('[')
            .ok_or_else(|| eyre!("Expected leading [ in {}", s))?
            .strip_suffix(']')
            .ok_or_else(|| eyre!("Expected trailing ] in {}", s))?
            .split(',')
            .map(|val| Ok(val.trim().parse()?))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|vec: Vec<_>| eyre!("Expected exactly 4 registers, got {}", vec.len()))?;
        Ok(Cpu { registers })
    }
}

#[derive(Debug, Clone, Copy)]
struct Sample {
    before: Cpu,
    instruction: UnknownInstruction,
    after: Cpu,
}

impl Sample {
    fn num_possible_ops(self) -> usize {
        all::<OpCode>()
            .filter(|opcode| {
                let result = self.before.execute(Instruction {
                    opcode: *opcode,
                    input_a: self.instruction.input_a,
                    input_b: self.instruction.input_b,
                    output_c: self.instruction.output_c,
                });
                self.after == result
            })
            .count()
    }
}

#[derive(Debug)]
struct Input {
    samples: Vec<Sample>,
    program: Vec<UnknownInstruction>,
}

impl FromStr for Input {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut samples = Vec::new();
        let mut lines = s.lines().peekable();
        while lines.peek().map(|line| !line.is_empty()).unwrap_or(false) {
            let before = lines.next().unwrap();
            let before = before
                .strip_prefix("Before:")
                .ok_or_else(|| eyre!("Expected before, got {}", before))?
                .trim()
                .parse()?;
            let instruction = lines
                .next()
                .ok_or_else(|| eyre!("Missing instruction in sample"))?;
            let instruction = instruction
                .parse()
                .wrap_err_with(|| format!("Expected instruction, got {}", instruction))?;
            let after = lines
                .next()
                .ok_or_else(|| eyre!("Missing after in sample"))?;
            let after = after
                .strip_prefix("After: ")
                .ok_or_else(|| eyre!("Expected after, got {}", after))?
                .trim()
                .parse()?;
            let blank = lines
                .next()
                .ok_or_else(|| eyre!("Expected blank line after sample"))?;
            if !blank.is_empty() {
                bail!("Expected blank line after sample, got {:?}", blank);
            }
            samples.push(Sample {
                before,
                instruction,
                after,
            });
        }
        lines.next();
        lines.next();
        let program = lines.map(|line| line.parse()).collect::<Result<_>>()?;
        Ok(Self { samples, program })
    }
}

#[instrument(skip(input))]
fn part1(input: &str) -> Result<String> {
    let input: Input = input.parse()?;
    let answer = input
        .samples
        .into_iter()
        .filter(|sample| sample.num_possible_ops() > 3)
        .count();
    Ok(answer.to_string())
}

type OpcodeSet = BitArr!(for 16, in u16);

#[instrument(skip(input))]
fn part2(input: &str) -> Result<String> {
    let input: Input = input.parse()?;
    let mut possible_opcodes: [OpcodeSet; 16] = [bitarr![u16, LocalBits; 1; 16]; 16];
    for sample in input.samples {
        for (i, opcode) in all::<OpCode>().enumerate() {
            if sample.after
                != sample.before.execute(Instruction {
                    opcode,
                    input_a: sample.instruction.input_a,
                    input_b: sample.instruction.input_b,
                    output_c: sample.instruction.output_c,
                })
            {
                possible_opcodes[sample.instruction.opcode as usize].set(i, false);
            }
        }
    }
    let mut opcode_map = [OpCode::AddI; 16];
    for _ in 0..16 {
        for i in 0..16 {
            let opcode = possible_opcodes[i];
            if opcode.count_ones() == 1 {
                opcode_map[i] = all::<OpCode>().nth(opcode.leading_zeros()).unwrap();
                for possible_opcodes in &mut possible_opcodes {
                    *possible_opcodes &= !opcode;
                }
            }
        }
    }

    let mut cpu = Cpu::default();
    for instruction in input.program {
        cpu = cpu.execute(Instruction {
            opcode: opcode_map[instruction.opcode as usize],
            input_a: instruction.input_a,
            input_b: instruction.input_b,
            output_c: instruction.output_c,
        });
    }
    Ok(cpu.registers[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_possible_ops() {
        let input: Input = "\
                Before: [3, 2, 1, 1]\n\
                9 2 1 2\n\
                After:  [3, 2, 2, 1]\n\
                \n\
            "
        .parse()
        .unwrap();
        assert_eq!(3, input.samples[0].num_possible_ops());
    }
}
