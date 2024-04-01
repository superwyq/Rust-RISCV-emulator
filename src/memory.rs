pub trait NUM {
    fn zero() -> Self;
}
impl NUM for u8 { fn zero() -> Self { 0 as u8}}
impl NUM for u16 {fn zero() -> Self {0 as u16}}
impl NUM for u32 {fn zero() -> Self {0 as u32}} 
//想要使用泛型将Memory中的数据类型变为u8,u16,u32，
//虽然这样很丑陋，但是才疏学浅，不知道有什么更好的方法

pub struct Memory<T: NUM+Copy,const N:usize> {
    //内存
    data: [T; N],
}

impl<T:NUM+Copy,const N:usize> Memory <T, N> {
    pub fn new() -> Memory<T,N> {
        Memory { data: [T::zero(); N] }
    }
    pub fn read(&self, address: u32) -> T {
        self.data[address as usize]
    }
    pub fn write(&mut self, address: u32, value: T) {
        self.data[address as usize] = value;
    }
    
}