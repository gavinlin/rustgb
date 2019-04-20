mod cpu;

use cpu::core::*;
use cpu::registers::*;

fn main() {
    println!("Hello, world!");
    let mut cpu = CPU::new();
    cpu.execute(Instruction::ADD(ByteTarget::C));
}
