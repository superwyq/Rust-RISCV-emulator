use rust_cpu::codegen::{compile_instruction, parse_instruction};
use std::io::BufRead;
use env_logger::Builder;
fn main() {
    println!("Hello, world!");
    Builder::new().filter(Some("rust_cpu::codegen"), log::LevelFilter::Debug).init();
    println!("please input the instruction:");
    let input_file_path = "test/test.asm";
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let input_file = std::io::BufReader::new(input_file);
    for line in input_file.lines() {
        let line = line.unwrap();
        let result = parse_instruction(&line);
        log::debug!("Parsed instruction: {:?}", result);
        match result {
            Ok((_, instruction)) => {
                let compiled = compile_instruction(&instruction);
                log::debug!("Compiled instruction: {:08X}", compiled);
            }
            Err(e) => {
                log::error!("Error: {:?}", e);
            }
        }
    }
}