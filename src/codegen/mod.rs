use nom::*;
use nom::{
    bytes::complete::{take_while1,is_not,tag}, // take_while1: 从输入中获取尽可能多的满足条件的字符
    character::complete::{char, multispace0}, //char: 从输入中获取一个字符, digit1: 从输入中获取尽可能多的数字字符, multispace0: 从输入中获取尽可能多的空格字符
    combinator::map_res, //map_res: 将解析器的输出映射到Result上
    sequence::preceded, //preceded: 匹配两个解析器，返回第二个解析器的结果
    IResult,
    error::ErrorKind,
    multi::separated_list0, //separated_list0: 将输入分割成由分隔符分隔的多个解析器，返回一个Ok包裹的元组，第一个元素是剩余的输入，第二个元素是解析器的输出
};
// Define a struct to represent a RISC-V instruction
#[derive(Debug)]
#[allow(dead_code)]
pub struct Instruction {
    opcode: String,
    operands: Vec<String>,
    rd: u8,
    rs1: u8,
    rs2: u8,
    immediate: u32,
}

// Parser for RISC-V instructions
pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, opcode) = take_while1(|c: char| c.is_alphabetic())(input)?;
    //take_while1: 从输入中获取尽可能多的满足条件的字符,这里的条件是c是char类型的，且是字母
    //take_while1返回值是一个元组，第一个元素是剩余的输入，第二个元素是获取到的字符
    let (input, _) = multispace0(input)?;
    //multispace0: 从输入中获取尽可能多的空格字符，返回值是一个元组，第一个元素是剩余的输入，第二个元素是获取到的字符
    let (input, operands) = separated_list0(
        preceded(multispace0, char(',')),
        map_res(preceded(multispace0,is_not(",")), |s: &str| Ok::<String,ErrorKind>(s.to_string())))(input)?;
    
    log::debug!("finish parsing operands: {:?}", operands);
    
    let mut register_parser = preceded(
        tag::<&str,&str,error::Error<&str>>("x"),
        map_res(
            take_while1::<_,&str,error::Error<&str>>(|c: char| c.is_digit(10)),
                |s: &str| Ok::<String, ErrorKind>(s.to_string()
            )
        )
    );

    let mut immediate_parser = map_res(
        take_while1::<_,&str,error::Error<&str>>(|c: char| c.is_digit(10)),
        |s: &str| Ok::<String, ErrorKind>(s.to_string()),
    );
    let mut instruction = Instruction {
        opcode: opcode.to_string(),
        operands: operands.iter().map(|s| s.to_string()).collect(),
        rd: 0,
        rs1: 0,
        rs2: 0,
        immediate: 0,
    };
    let mut rs_parse_id = 0;
    for operand in &operands {
        if register_parser(operand).is_ok() {
            let register_id = register_parser(operand).unwrap().1.parse::<u8>().unwrap();
            if rs_parse_id == 0{
                log::debug!("rd: {}", operand);
                instruction.rd = register_id;
                rs_parse_id += 1;
            } else if rs_parse_id == 1{
                log::debug!("rs1: {}", operand);
                instruction.rs1 = register_id;
            } else if rs_parse_id == 2{
                log::debug!("rs2: {}", operand);
                instruction.rs2 = register_id;
            } else {
                log::error!("{}: most 3 register,get 4", operand);
                return Err(nom::Err::Error(nom::error::Error::new(input, ErrorKind::Tag)));
            }
        }else if immediate_parser(operand).is_ok() {
            let imm = immediate_parser(operand).unwrap().1.parse::<u32>().unwrap();
            log::debug!("immediate: {}", operand);
            instruction.immediate = imm;
        }else{
            log::error!("Unknown operand: {}", operand);
            return Err(nom::Err::Error(nom::error::Error::new(input, ErrorKind::Tag)));
        }
    }
    Ok((
        input,
        instruction,
    ))
}

pub fn compile_instruction(instruction: &Instruction) -> u32 {
    let mut result:u32 = 0;
    match instruction.opcode.as_str() {
        "add" => {
            result |= 0b0110011;
            result |= (instruction.rd as u32) << 7;
            result |= (instruction.rs1 as u32) << 15;
            result |= (instruction.rs2 as u32) << 20;
            result |= 0b0000000 << 25;
        }
        "sub" => {
            result |= 0b0110011;
            result |= (instruction.rd as u32) << 7;
            result |= (instruction.rs1 as u32) << 15;
            result |= (instruction.rs2 as u32) << 20;
            result |= 0b0100000 << 25;
        }
        "addi" => {
            result |= 0b0010011;
            result |= (instruction.rd as u32) << 7;
            result |= (instruction.rs1 as u32) << 15;
            result |= instruction.immediate << 20;
        }
        _ => {
            log::error!("Unknown opcode: {}", instruction.opcode);
        }
    }
    result
}