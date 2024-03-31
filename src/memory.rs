pub struct Memory {
    //内存
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; 4096] }
    }
    pub fn read(&self, address: u32) -> u8 {
        self.data[address as usize]
    }
    pub fn write(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }
    
}