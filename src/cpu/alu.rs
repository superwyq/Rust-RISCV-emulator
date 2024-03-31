pub struct ALU {
    //算术逻辑单元
}
impl ALU {
    pub fn new() -> ALU {
        ALU {}
    }
    pub fn add(&self, a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }
    pub fn sub(&self, a: u32, b: u32) -> u32 {
        a.wrapping_sub(b)
    }
}