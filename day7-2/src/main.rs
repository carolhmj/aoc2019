use std::io::{Read};
use std::fs::File;
use std::collections::{HashSet, LinkedList};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

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

fn set_input(memory : &mut Vec<i32>, pc : usize, instruction : Instruction, inputs : &Receiver<i32>) -> Option<usize> {
    // println!("Waiting for inputs...");
    let value = inputs.recv();

    if value.is_ok() {
        // println!("[{:?}] Received input {:?}", thread::current().name(), value);
        input(memory, pc, instruction, value.unwrap())
    } else {
        // println!("[{:?}] Error on input {:?}",  thread::current().name(), value);
        None
    }
    
}

fn output(memory : &Vec<i32>, pc : usize, instruction : Instruction, output : &mut i32) -> Option<usize> {
    let value = get_value(memory, pc+1, instruction.mode1);
    
    *output = value;
    
    Some(pc + 2)
}

fn set_output(memory : &Vec<i32>, pc : usize, instruction : Instruction, outputs : &Sender<i32>) -> Option<usize> {
    let mut out : i32 = 0; 
    let res = output(&memory, pc, instruction, &mut out);
    
    let send_res = outputs.send(out);

    if send_res.is_ok() {
        // println!("[{:?}] sent output {:?}",  thread::current().name(), send_res);
        res
    } else {
        // println!("[{:?}] Error on output {:?}",  thread::current().name(), send_res);
        None
    }
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

fn run_code(memory : &mut Vec<i32>, inputs : Receiver<i32>, outputs : Sender<i32>, save_outputs : bool) -> Option<Vec<i32>> {
    let mut pc = 0;
    
    while pc < memory.len() {
        let instruction = decode(memory[pc]);
        // println!("[{:?}] Instruction {:?}", thread::current().name(), instruction);
        let new_pc = match instruction.opcode {
            99 => None,
            1 | 2 => fetch_operands_and_store_result(memory, pc, instruction),
            3 => set_input(memory, pc, instruction, &inputs),
            4 => set_output(memory, pc, instruction, &outputs),
            5 | 6 => jump_if(memory, pc, instruction),
            7 | 8 => comparison(memory, pc, instruction),
            _ => panic!("Unexpected opcode"),
        };
        
        match new_pc {
            Some(new_pc) => {pc = new_pc;}
            None => {
                if save_outputs {
                    // Get remaining values from receiver and return
                    let mut rem = vec![];
                    for rem_rec in inputs.iter() {
                        rem.push(rem_rec);
                    }
                    // println!("[{:?}] Finished code run with {:?}", thread::current().name(), rem);
                    return Some(rem);
                } else {
                    // println!("[{:?}] Finished code run", thread::current().name());
                    return None;
                }
            }
        }
    }

    return None;
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
    vals.insert(5);
    vals.insert(6);
    vals.insert(7);
    vals.insert(8);
    vals.insert(9);
    
    // println!("{:?}", generate_combinations(vals));
    let mut max = i32::min_value();
    let mut ampl_seq = None;
    
    // let thruster_seq
    for mut possible_seq in generate_combinations(vals) {
        // println!("Possible sequence {:?}", possible_seq);
        let ampl_a = possible_seq.pop_front().unwrap();
        let ampl_b = possible_seq.pop_front().unwrap();
        let ampl_c = possible_seq.pop_front().unwrap();
        let ampl_d = possible_seq.pop_front().unwrap();
        let ampl_e = possible_seq.pop_front().unwrap();
        
        let (a_sender, a_receiver) = channel();
        let (b_sender, b_receiver) = channel();
        let (c_sender, c_receiver) = channel();
        let (d_sender, d_receiver) = channel();
        let (e_sender, e_receiver) = channel();

        // Phase setting
        a_sender.send(ampl_a).unwrap();
        b_sender.send(ampl_b).unwrap();
        c_sender.send(ampl_c).unwrap();
        d_sender.send(ampl_d).unwrap();
        e_sender.send(ampl_e).unwrap();
        // First signal
        a_sender.send(0).unwrap();

        // println!("Sent phase settings and first signal");

        let mut mem = initial_memory.clone();
        let thread_a = thread::Builder::new().name("amplifier a".to_string()).spawn(move || {
            run_code(&mut mem, a_receiver, b_sender, true) 
        }).unwrap();

        let mut mem = initial_memory.clone();
        let thread_b = thread::Builder::new().name("amplifier b".to_string()).spawn(move || {
            run_code(&mut mem, b_receiver, c_sender, false)
            // run_code(&mut mem, b_receiver, a_sender, false)
        }).unwrap();

        let mut mem = initial_memory.clone();
        let thread_c = thread::Builder::new().name("amplifier c".to_string()).spawn(move || {
            run_code(&mut mem, c_receiver, d_sender, false)
        }).unwrap();

        let mut mem = initial_memory.clone();
        let thread_d = thread::Builder::new().name("amplifier d".to_string()).spawn(move || {
            run_code(&mut mem, d_receiver, e_sender, false)
        }).unwrap();

        let mut mem = initial_memory.clone();
        let thread_e = thread::Builder::new().name("amplifier e".to_string()).spawn(move || {
            run_code(&mut mem, e_receiver, a_sender, false)
        }).unwrap();
        
        thread_e.join().unwrap();
        thread_d.join().unwrap();
        thread_c.join().unwrap();
        thread_b.join().unwrap();
        let res = match thread_a.join() {
            Ok(Some(v)) => v[0],
            _ => panic!("Received no value in thread finish")
        };

        if res > max {
            max = res;
            ampl_seq = Some([ampl_a, ampl_b, ampl_c, ampl_d, ampl_e]);
            // ampl_seq = Some([ampl_a, ampl_b]);
        }
    }
    println!("The maximum signal is {:?} for the input sequence {:?}", max, ampl_seq.unwrap());                            
}
                        