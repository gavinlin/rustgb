use super::registers::*;

pub enum Instruction {
    LD(ByteTarget, ByteTarget),
    ADD(ByteTarget),
    ADC(ByteTarget),
    SUB(ByteTarget),
    SBC(ByteTarget),
}


pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; 0xFFFF]
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {

    pub fn new() -> CPU {
        CPU{
            registers: Registers::new(),
            pc: 0,
            bus: MemoryBus::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LD(from, target) => {
                let value = self.registers.get_byte(from);
                self.registers.set_byte(target, value);
            },
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

    fn cp(&mut self, value: u8) {
        self.alu_sub(value, false);
    }

    fn and(&mut self, value: u8) {
        let new_value = self.registers.get_byte(ByteTarget::A) & value;
        self.registers.set_byte(ByteTarget::A, new_value);
        self.registers.set_flag(new_value == 0, false, true, false);
    }

    fn or(&mut self, value: u8) {
        let new_value = self.registers.get_byte(ByteTarget::A) | value;
        self.registers.set_byte(ByteTarget::A, new_value);
        self.registers.set_flag(new_value == 0, false, false, false);
    }

    fn xor(&mut self, value: u8) {
        let new_value = self.registers.get_byte(ByteTarget::A) ^ value;
        self.registers.set_byte(ByteTarget::A, new_value);
        self.registers.set_flag(new_value == 0, false, false, false);
    }

    fn inc(&mut self, value: u8, out: ByteTarget) {
        let new_value = value.wrapping_add(1);
        let half_carry = value & 0xF == 0xF;
        let carry = self.registers.get_carry();
        self.registers.set_flag(new_value == 0, false, carry, half_carry);
        self.registers.set_byte(out, new_value);
    }

    fn dec(&mut self, value: u8, out: ByteTarget) {
        let new_value = value.wrapping_sub(1);
        let half_carry = value & 0xF == 0;
        let carry = self.registers.get_carry();
        self.registers.set_flag(new_value == 0, true, carry, half_carry);
        self.registers.set_byte(out, new_value);
    }

    fn rlca(&mut self) {
        let value = self.registers.get_byte(ByteTarget::A);
        let new_value = self.alu_rlc(value, false);
        self.registers.set_byte(ByteTarget::A, new_value);
    }

    fn rla(&mut self) {
        let value = self.registers.get_byte(ByteTarget::A);
        let new_value = self.alu_rl(value, false);
        self.registers.set_byte(ByteTarget::A, new_value);
    }

    fn rrca(&mut self) {
        let value = self.registers.get_byte(ByteTarget::A);
        let new_value = self.alu_rrc(value, false);
        self.registers.set_byte(ByteTarget::A, new_value);
    }

    fn rra(&mut self) {
        let value = self.registers.get_byte(ByteTarget::A);
        let new_value = self.alu_rr(value, false);
        self.registers.set_byte(ByteTarget::A, new_value);
    }

    fn rlc(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let new_value = self.alu_rlc(value, true);
        self.registers.set_byte(from, new_value);
    }

    fn rl(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let new_value = self.alu_rl(value, true);
        self.registers.set_byte(from, new_value);
    }

    fn rrc(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let new_value = self.alu_rrc(value, true);
        self.registers.set_byte(from, new_value);
    }

    fn rr(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let new_value = self.alu_rr(value, true);
        self.registers.set_byte(from, new_value);
    }

    fn sla(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let co = value & 0x80;
        let new_value = value << 1;
        self.registers.set_flag(
            new_value == 0,
            false,
            co != 0,
            false);
        self.registers.set_byte(from, new_value);
    }

    fn sra(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let co = value & 0x01;
        let hi = value & 0x80;
        let new_value = (value >> 1) | hi;
        self.registers.set_flag(
            new_value == 0,
            false,
            co != 0,
            false);
        self.registers.set_byte(from, new_value);
    }

    fn srl(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let co = value & 0x01;
        let new_value = value >> 1;
        self.registers.set_flag(
            new_value == 0,
            false,
            co != 0,
            false);
        self.registers.set_byte(from, new_value);
    }

    fn swap(&mut self, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let new_value = (value >> 4) | (value << 4);
        self.registers.set_flag(
            new_value == 0,
            false,
            false,
            false);
        self.registers.set_byte(from, new_value);
    }

    fn bit(&mut self, bit: usize, from: ByteTarget) {
        let value = self.registers.get_byte(from);
        let new_value = value & (1 << bit);
        let carry = self.registers.get_carry();
        self.registers.set_flag(
            new_value == 0,
            false,
            carry,
            true);
    }

    fn set(&mut self, bit: usize, from: ByteTarget) {

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

    fn alu_rl(&mut self, value: u8, set_zero: bool) -> u8 {
        let ci = if (self.registers.get_carry()) { 1 } else { 0 };
        let co = value & 0x80;
        let new_value = (value << 1) | ci;
        self.registers.set_flag(
            set_zero && new_value == 0,
            false,
            co != 0,
            false
        );
        new_value
    }

    fn alu_rlc(&mut self, value: u8, set_zero: bool) -> u8 {
        let co = value & 0x80;
        let new_value = value.rotate_left(1);
        self.registers.set_flag(set_zero && new_value == 0, false, co != 0, false);
        new_value
    }

    fn alu_rrc(&mut self, value: u8, set_zero: bool) -> u8 {
        let co = value & 0x01;
        let new_value = value.rotate_right(1);
        self.registers.set_flag(set_zero && new_value == 0, false, co != 0, false);
        new_value
    }

    fn alu_rr(&mut self, value: u8, set_zero: bool) -> u8 {
        let ci = if (self.registers.get_carry()) { 1 } else { 0 };
        let co = value & 0x01;
        let new_value = (value >> 1) | (ci << 7);
        self.registers.set_flag(
            set_zero && new_value == 0,
            false,
            co != 0,
            false);
        new_value
    }
}

