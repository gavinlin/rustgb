use super::registers::*;

pub enum Instruction {
    ADD(ByteTarget)
}


pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
}

impl CPU {

    pub fn new() -> CPU {
        CPU{
            registers: Registers::new(),
            pc: 0
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ByteTarget::C => {
                        let value = self.registers.get_byte(target);
                        let new_value = self.add(value);
                        self.registers.set_byte(new_value, target);
                    }

                    _ => { /* TODO: more target */}
                }
            }
            _ => { /* TODO: more instruction */}
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let mut registers = self.registers;
        let (new_value, did_overflow) = registers.get_byte(ByteTarget::A).overflowing_add(value);
        self.registers.set_flag(new_value == 0, false, did_overflow,
                                (registers.get_byte(ByteTarget::A) & 0xF) + (value & 0xF) > 0xF);
        new_value
    }
}
