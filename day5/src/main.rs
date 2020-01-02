use std::io::{self, Read};
use std::fs::File;

#[derive(Debug)]
struct Instruction {
    opcode : i32,
    mode1 : i32,
    mode2 : i32,
    mode3 : i32,
}

fn decode (i : i32) -> Instruction {
    let mut i = i;
    
    let opcode = i % 100;
    
    i /= 100;
    
    let mode1 = i % 10;
    
    i /= 10;
    
    let mode2 = i % 10;
    
    i /= 10;
    
    let mode3 = i % 10;
    
    return Instruction {
        opcode,
        mode1,
        mode2, 
        mode3
    }
}

fn get_value(memory : &Vec<i32>, position : usize, mode : i32) -> i32 {
    let memory_value = memory[position] as usize;
    // POSITION MODE
    if mode == 0 {
        memory[memory_value]
    } else {
        memory_value as i32
    }
}

fn store_value(memory : &mut Vec<i32>, position : usize, value : i32, mode : i32) {
    let memory_value = memory[position] as usize;
    // POSITION MODE
    if mode == 0 {
        memory[memory_value] = value;
    } else {
        panic!("Unexpected mode for a store operation");
    }
}

fn fetch_operands_and_store_result(memory : &mut Vec<i32>, pc : usize, instruction : Instruction) -> Option<usize> {
    
    let op1 = get_value(memory, pc+1, instruction.mode1);
    let op2 = get_value(memory, pc+2, instruction.mode2);
    
    let result : i32 = match instruction.opcode {
        1 => op1 + op2,
        2 => op1 * op2,
        _ => panic!("Unexpected opcode")
    };

    store_value(memory, pc+3, result, instruction.mode3);

    Some(pc + 4)
}

fn input(memory : &mut Vec<i32>, pc : usize, instruction : Instruction) -> Option<usize> {
    let mut buffer = String::new();
    
    println!("Please input a value");
    io::stdin().read_line(&mut buffer).unwrap();
    
    let result = i32::from_str_radix(&buffer.trim(), 10).unwrap();
    store_value(memory, pc+1, result, instruction.mode1);

    Some(pc + 2)
}

fn output(memory : &Vec<i32>, pc : usize, instruction : Instruction) -> Option<usize> {
    let value = get_value(memory, pc+1, instruction.mode1);

    println!("Output value {:?}", value);

    Some(pc + 2)
}

fn jump_if(memory : &Vec<i32>, pc : usize, instruction : Instruction) -> Option<usize> {
    let op1 = get_value(memory, pc+1, instruction.mode1);

    let result = match instruction.opcode {
        5 => op1 != 0,
        6 => op1 == 0,
        _ => panic!("Unexpected opcode"),
    };

    let op2 = get_value(memory, pc+2, instruction.mode2);

    if result {
        Some(op2 as usize)
    } else {
        Some(pc+3)
    }
}

fn comparison(memory : &mut Vec<i32>, pc : usize, instruction : Instruction) -> Option<usize> {
    let op1 = get_value(memory, pc+1, instruction.mode1);
    let op2 = get_value(memory, pc+2, instruction.mode2);

    let result = match instruction.opcode {
        7 => op1 < op2,
        8 => op1 == op2,
        _ => panic!("Unexpected opcode"),
    };

    let value = if result { 1 } else { 0 };

    store_value(memory, pc+3, value, instruction.mode3);

    Some(pc+4)
}

fn read_input(filename : &str) -> Vec<i32> {
    let mut contents = String::new();

    File::open(filename)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    contents.split(',')
            .map(|n| i32::from_str_radix(&n.trim(), 10)
                     .unwrap()
                )
            .collect()    
}

fn main() {
    let mut memory = read_input("input.txt");

    println!("Welcome to the INTCODE computer!");
    
    let mut pc = 0;
    
    while pc < memory.len() {
        let instruction = decode(memory[pc]);
        let new_pc = match instruction.opcode {
            99 => None,
            1 | 2 => fetch_operands_and_store_result(&mut memory, pc, instruction),
            3 => input(&mut memory, pc, instruction),
            4 => output(&memory, pc, instruction),
            5 | 6 => jump_if(&memory, pc, instruction),
            7 | 8 => comparison(&mut memory, pc, instruction),
            _ => panic!("Unexpected opcode"),
        };
        
        match new_pc {
            Some(new_pc) => {pc = new_pc;}
            None => {
                println!("Halt! Value at pos 0 is {:?}", memory[0]); 
                return;
            }
        }
    }
}
