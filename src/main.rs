pub mod  cpu;
pub mod memory;
fn main() {
    println!("Hello, world!");
    let mut cpu = cpu::CPU::new();
    let instruction:u32 = 0b00010000000000000000000010010011;
    cpu.memory.write(0x200, (instruction & 0xFF) as u8);
    cpu.memory.write(0x201, ((instruction >> 8) & 0xFF) as u8);
    cpu.memory.write(0x202, ((instruction >> 16) & 0xFF) as u8);
    cpu.memory.write(0x203, ((instruction >> 24) & 0xFF) as u8);

    cpu.run(true);
}
