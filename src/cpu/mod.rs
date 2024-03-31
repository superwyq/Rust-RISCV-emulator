pub mod register;
pub mod alu;

use register::RegisterFile;
use crate::memory::Memory;
use alu::ALU;


#[allow(dead_code)]
pub struct CPU {
    zero:[u8; 1],           
    //0寄存器，用于存储0
    pub registers: RegisterFile,    
    //32个通用寄存器,RISC-V使用小端序，小端序是指低位字节存放在低地址，高位字节存放在高地址，字节内部的位的顺序与大小端序无关
    pub memory: Memory,     
    //4KB内存，用于存储指令和数据
    pub pc: u32,                
    //程序计数器，指向当前指令
    pub alu: ALU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            zero: [0; 1],
            registers: RegisterFile::new(),
            memory: Memory::new(),
            pc: 0x200, //RISC-V默认初始化地址我也不知道，我就随便写了一个
            alu: ALU::new(),
        }
    }

    pub fn read_instruction(&mut self) -> u32 {
        let pc = self.pc;
        let byte1 = self.memory.read(pc) as u32;
        let byte2 = self.memory.read(pc + 1) as u32;
        let byte3 = self.memory.read(pc + 2) as u32;
        let byte4 = self.memory.read(pc + 3) as u32;
        self.pc += 4;
        byte4 << 24 | byte3 << 16 | byte2 << 8 | byte1
        //小端序，将4个字节拼接成一个32位整数
    }

    pub fn run(&mut self,step:bool) {
        loop {
            let instruction = self.read_instruction();
            let opcode = instruction & 0x7F;
            
            match opcode {
                0x33 => {
                    let rs1 = (instruction >> 15) & 0x1F;
                    let rs2 = (instruction >> 20) & 0x1F;
                    let rd = (instruction >> 7) & 0x1F;

                    let funct7 = (instruction >> 25) & 0x7F;
                    match funct7 {
                        
                        0x00 => { //add
                            println!("add");
                            let result = self.alu.add(self.registers.read(rs1), self.registers.read(rs2));
                            self.registers.write(rd, result);
                        }
                        0x20 => { //sub
                            println!("sub");
                            let result = self.alu.sub(self.registers.read(rs1), self.registers.read(rs2));
                            self.registers.write(rd, result);
                        }
                        _ => {
                            println!("Unknown funct7: {:02X}", funct7);
                            break;
                        }
                    }
                }
                0x13 => {
                    println!("addi");
                    let rs1 = (instruction >> 15) & 0x1F;
                    let rd = (instruction >> 7) & 0x1F;
                    let imm = ((instruction >> 20) & 0xFFF) | ((instruction >> 31) << 11);
                    let result = self.alu.add(self.registers.read(rs1), imm);
                    println!("rs1:{},rd:{},imm:{},result:{}",rs1,rd,imm,result);
                    self.registers.write(rd, result);
                }
                _ => {
                    println!("Unknown opcode: {:02X}", opcode);
                    break;
                }
            }
            if step {
                self.debug();
                println!("Press Enter to continue...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "q" {
                    break;
                    
                }
            }
            
        }
    }
    pub fn debug(&self) -> (){
        self.registers.debug();
    }
}