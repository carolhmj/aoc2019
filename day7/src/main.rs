use std::io::{Read};
use std::fs::File;
use std::collections::{HashSet, LinkedList};

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

fn input(memory : &mut Vec<i32>, pc : usize, instruction : Instruction, input : i32) -> Option<usize> {
    store_value(memory, pc+1, input, instruction.mode1);
    
    Some(pc + 2)
}

fn set_input(memory : &mut Vec<i32>, pc : usize, instruction : Instruction, inputs : &Vec<i32>, input_pos : &mut usize) -> Option<usize> {
    let value = inputs[*input_pos];
    
    *input_pos += 1;
    
    input(memory, pc, instruction, value)
}

fn output(memory : &Vec<i32>, pc : usize, instruction : Instruction, output : &mut i32) -> Option<usize> {
    let value = get_value(memory, pc+1, instruction.mode1);
    
    *output = value;
    
    Some(pc + 2)
}

fn set_output(memory : &Vec<i32>, pc : usize, instruction : Instruction, outputs : &mut Vec<i32>) -> Option<usize> {
    let mut out : i32 = 0; 
    let res = output(&memory, pc, instruction, &mut out);
    
    outputs.push(out);
    
    res
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

fn run_code(memory : &mut Vec<i32>, inputs : Vec<i32>) -> Vec<i32> {
    let mut outputs = Vec::<i32>::new();
    
    let mut pc = 0;
    let mut input_counter = 0;
    
    while pc < memory.len() {
        let instruction = decode(memory[pc]);
        let new_pc = match instruction.opcode {
            99 => None,
            1 | 2 => fetch_operands_and_store_result(memory, pc, instruction),
            3 => set_input(memory, pc, instruction, &inputs, &mut input_counter),
            4 => set_output(memory, pc, instruction, &mut outputs),
            5 | 6 => jump_if(memory, pc, instruction),
            7 | 8 => comparison(memory, pc, instruction),
            _ => panic!("Unexpected opcode"),
        };
        
        match new_pc {
            Some(new_pc) => {pc = new_pc;}
            None => {
                return outputs;
            }
        }
    }
    
    return outputs;
}

fn generate_combinations(possible_inputs : HashSet<i32>) -> Vec<LinkedList<i32>> {
    if possible_inputs.len() > 0 {
        return possible_inputs.iter().flat_map(|input| {
            let mut inputs_clon = possible_inputs.clone();
            inputs_clon.remove(input);
            
            generate_combinations(inputs_clon).into_iter().map(move |mut comb| {
                comb.push_front(*input);
                comb
            })
        }).collect::<Vec<LinkedList<i32>>>();
    }
    return vec![LinkedList::new()];
}

fn main() {
    let initial_memory = read_input("input.txt");
    
    println!("Welcome to the INTCODE computer!");
    
    
    let mut vals = HashSet::new();
    vals.insert(0);
    vals.insert(1);
    vals.insert(2);
    vals.insert(3);
    vals.insert(4);
    
    // println!("{:?}", generate_combinations(vals));
    let mut max = i32::min_value();
    let mut ampl_seq = [0,1,2,3,4];
    
    // let thruster_seq
    for mut possible_seq in generate_combinations(vals) {
        
        let ampl_a = possible_seq.pop_front().unwrap();
        let ampl_b = possible_seq.pop_front().unwrap();
        let ampl_c = possible_seq.pop_front().unwrap();
        let ampl_d = possible_seq.pop_front().unwrap();
        let ampl_e = possible_seq.pop_front().unwrap();
        
        let res_a = run_code(&mut initial_memory.clone(), vec![ampl_a, 0]);
        let res_b = run_code(&mut initial_memory.clone(), vec![ampl_b, res_a[0]]);
        let res_c = run_code(&mut initial_memory.clone(), vec![ampl_c, res_b[0]]);
        let res_d = run_code(&mut initial_memory.clone(), vec![ampl_d, res_c[0]]);
        let res_e = run_code(&mut initial_memory.clone(), vec![ampl_e, res_d[0]]);
        
        if res_e[0] > max {
            max = res_e[0];
            ampl_seq = [ampl_a, ampl_b, ampl_c, ampl_d, ampl_e];
        }
    }
    println!("The maximum signal is {:?} for the input sequence {:?}", max, ampl_seq);                            
}
                        