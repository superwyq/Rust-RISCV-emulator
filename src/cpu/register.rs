pub struct RegisterFile {
    //寄存器文件
    registers: [u32; 32],
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile { registers: [0; 32] }
    }
    pub fn read(&self, register: u32) -> u32 {
        self.registers[register as usize]
    }
    pub fn write(&mut self, register: u32, value: u32) {
        if register != 0 {
            self.registers[register as usize] = value;
        }
    }
    pub fn debug(&self) {
        for i in 0..32 {
            println!("x{:02}: {:08X}", i, self.registers[i]);
        }
    }
}