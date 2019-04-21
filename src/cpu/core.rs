use super::registers::*;

pub enum Instruction {
    ADD(ByteTarget),
    ADC(ByteTarget),
    SUB(ByteTarget),
    SBC(ByteTarget),
}


pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
}

pub trait CpuOps {
    type R;
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
                let value = self.registers.get_byte(target);
                self.add(value);
            },
            Instruction::ADC(target) => {
                let value = self.registers.get_byte(target);
                self.adc(value);
            }
            Instruction::SUB(target) => {
                let value = self.registers.get_byte(target);
                self.sub(value);
            }
            Instruction::SBC(target) => {
                let value = self.registers.get_byte(target);
                self.sbc(value);
            }
            _ => { /* TODO: more instruction */}
        }
    }

    fn add(&mut self, value: u8) {
        let mut registers = self.registers;
        let (new_value, did_overflow) = registers.get_byte(ByteTarget::A).overflowing_add(value);
        self.registers.set_flag(new_value == 0, false, did_overflow,
                                (registers.get_byte(ByteTarget::A) & 0xF) + (value & 0xF) > 0xF
        );
        registers.set_byte(ByteTarget::A, new_value);
    }

    fn adc(&mut self, value: u8) {
        let mut registers = self.registers;
        let cy = if (registers.get_carry()) { 1 } else { 0 };
        let result = registers.get_byte(ByteTarget::A).wrapping_add(value).wrapping_add(cy);
        registers.set_flag(result == 0, false,
                           registers.get_byte(ByteTarget::A) as u16 + value as u16 + cy as u16 > 0xFF,
                           (registers.get_byte(ByteTarget::A) & 0xF) + (value & 0xF) + cy > 0xF
        );
        registers.set_byte(ByteTarget::A, result);
    }

    fn sub(&mut self, value: u8) {
        let result = self.alu_sub(value, false);
        self.registers.set_byte(ByteTarget::A, result);
    }

    fn sbc(&mut self, value: u8) {
        let result = self.alu_sub(value, true);
        self.registers.set_byte(ByteTarget::A, result);
    }

    fn alu_sub(&mut self, value: u8, use_carry: bool) -> u8 {
        let mut regs = self.registers;
        let cy = if use_carry && regs.get_carry() { 1 } else { 0 };

        let result = regs.get_byte(ByteTarget::A).wrapping_sub(value).wrapping_sub(cy);
        regs.set_flag(result == 0,
                      true,
                      (regs.get_byte(ByteTarget::A) as u16) < (value as u16) + (cy as u16),
                      (regs.get_byte(ByteTarget::A) & 0xF) < (value & 0xF) + cy);
        result
    }
}
