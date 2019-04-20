mod cpu;

use cpu::registers::Registers;

fn main() {
    println!("Hello, world!");

    let mut register = Registers::new();
    register.c = 2;
    println!("Get register a is {}", register.get_bc());
}
