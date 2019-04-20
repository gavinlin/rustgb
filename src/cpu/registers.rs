pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
}

pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

const ZERO_POSITION: u8 = 7;
const SUBTRACT_POSITION: u8 = 6;
const HALF_CARRY_POSITION: u8 = 5;
const CARRY_POSITION: u8 = 4;

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
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
        }
    }


    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
}
