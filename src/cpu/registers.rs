#[derive(Clone, Copy, Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool
}

const ZERO_POSITION: u8 = 7;
const SUBTRACT_POSITION: u8 = 6;
const HALF_CARRY_POSITION: u8 = 5;
const CARRY_POSITION: u8 = 4;

#[derive(Clone, Copy, Debug)]
pub enum ByteTarget {
    A, B, C, D, E, F, H, L,
}

#[derive(Clone, Copy, Debug)]
pub enum WordTarget {
    BC, DE, HL, AF
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_POSITION |
        (if flag.subtract { 1 } else { 0 }) << SUBTRACT_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_POSITION |
        (if flag.carry { 1 } else { 0 }) << CARRY_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::from(0),
            h: 0,
            l: 0,
        }
    }

    pub fn set_byte(&mut self, value: u8, byte_target: ByteTarget) {
        match byte_target {
            ByteTarget::A => {
                self.a = value;
            }
            ByteTarget::B => {
                self.b = value;
            }
            ByteTarget::C => {
                self.c = value;
            }
            ByteTarget::D => {
                self.d = value;
            }
            ByteTarget::E => {
                self.e = value;
            }
            ByteTarget::F => {
                self.f = FlagsRegister::from(value)
            }
            ByteTarget::H => {
                self.h = value;
            }
            ByteTarget::L => {
                self.l = value;
            }
        }
    }

    pub fn get_byte(&mut self, byte_target: ByteTarget) -> u8 {
        match byte_target {
            ByteTarget::A => self.a,
            ByteTarget::B => self.b,
            ByteTarget::C => self.c,
            ByteTarget::D => self.d,
            ByteTarget::E => self.e,
            ByteTarget::F => u8::from(self.f),
            ByteTarget::H => self.h,
            ByteTarget::L => self.l
        }
    }

    pub fn set_flag(&mut self, zero: bool, subtract: bool, carry: bool, half_carry: bool) {
        self.f.zero = zero;
        self.f.subtract = subtract;
        self.f.carry = carry;
        self.f.half_carry = half_carry;
    }

    pub fn set_word(&mut self, value: u16, word_target: WordTarget) {
        match word_target {
            WordTarget::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0x00FF) as u8;
            }
            WordTarget::DE => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0x00FF) as u8;
            }
            WordTarget::AF => {
                self.a = ((value & 0xFF00) >> 8) as u8;
                self.f = FlagsRegister::from((value & 0x00FF) as u8);
            }
            WordTarget::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0x00FF) as u8;
            }
        }
    }

    pub fn get_word(&mut self, word_target: WordTarget) -> u16 {
        match word_target {
            WordTarget::BC => (self.b as u16) << 8 | self.c as u16,
            WordTarget::DE => (self.d as u16) << 8 | self.e as u16,
            WordTarget::AF => (self.a as u16) << 8 | u8::from(self.f) as u16,
            WordTarget::HL => (self.h as u16) << 8 | self.l as u16
        }
    }
}
