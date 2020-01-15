use std::io::{self, Read};
use std::fs::File;
use std::collections::HashMap;
use std::f64;

type Num = i64;
type Pos = (Num, Num);

#[derive(Debug)]
struct Instruction {
    opcode : Num,
    mode1 : Num,
    mode2 : Num,
    mode3 : Num,
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W
}

fn decode (i : Num) -> Instruction {
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

fn safe_get(memory : &mut Vec<Num>, position : usize) -> Num {
    if position >= memory.len() {
        memory.resize(position + 1, 0);
    }
    memory[position]
}

fn safe_set(memory : &mut Vec<Num>, position : usize, value : Num) {
    if position >= memory.len() {
        memory.resize(position + 1, 0);
    }
    memory[position] = value;
}

fn get_value(memory : &mut Vec<Num>, 
             position : usize, 
             mode : Num,
             relative_base: Num) -> Num {

    let memory_value = safe_get(memory, position);
    if mode == 0 { // POSITION MODE
        safe_get(memory, memory_value as usize)
    } else if mode == 1 { // IMMEDIATE MODE
        memory_value as Num
    } else { // RELATIVE MODE
        safe_get(memory, (memory_value + relative_base) as usize)
    }
}

fn store_value(memory : &mut Vec<Num>, 
               position : usize, 
               value : Num, 
               mode : Num,
               relative_base : Num) {
    let memory_value = safe_get(memory, position);
    if mode == 0 { // POSITION MODE
        safe_set(memory, memory_value as usize, value);
    } else if mode == 2 {
        safe_set(memory, (memory_value + relative_base) as usize, value);
    } else {
        panic!("Unexpected mode for a store operation");
    }
}

fn fetch_operands_and_store_result(memory : &mut Vec<Num>, 
                                   pc : usize, 
                                   instruction : Instruction,
                                   relative_base : Num) -> Option<usize> {
    
    let op1 = get_value(memory, pc+1, instruction.mode1, relative_base);
    let op2 = get_value(memory, pc+2, instruction.mode2, relative_base);
    
    let result : Num = match instruction.opcode {
        1 => op1 + op2,
        2 => op1 * op2,
        _ => panic!("Unexpected opcode")
    };

    store_value(memory, pc+3, result, instruction.mode3, relative_base);

    Some(pc + 4)
}

fn input(memory : &mut Vec<Num>, 
        pc : usize, 
        instruction : Instruction, 
        relative_base : Num, 
        value : Num) -> Option<usize> {
            
    store_value(memory, pc+1, value, instruction.mode1, relative_base);

    Some(pc + 2)
}

fn output(memory : &mut Vec<Num>, 
          pc : usize, 
          instruction : Instruction,
          relative_base : Num,
          value : &mut Num
        ) -> Option<usize> {

    *value = get_value(memory, pc+1, instruction.mode1, relative_base);

    Some(pc + 2)
}

fn jump_if(memory : &mut Vec<Num>, 
           pc : usize, 
           instruction : Instruction,
           relative_base : Num) -> Option<usize> {
    let op1 = get_value(memory, pc+1, instruction.mode1, relative_base);

    let result = match instruction.opcode {
        5 => op1 != 0,
        6 => op1 == 0,
        _ => panic!("Unexpected opcode"),
    };

    let op2 = get_value(memory, pc+2, instruction.mode2, relative_base);

    if result {
        Some(op2 as usize)
    } else {
        Some(pc+3)
    }
}

fn comparison(memory : &mut Vec<Num>, 
              pc : usize, 
              instruction : Instruction,
              relative_base : Num) -> Option<usize> {
    let op1 = get_value(memory, pc+1, instruction.mode1, relative_base);
    let op2 = get_value(memory, pc+2, instruction.mode2, relative_base);

    let result = match instruction.opcode {
        7 => op1 < op2,
        8 => op1 == op2,
        _ => panic!("Unexpected opcode"),
    };

    let value = if result { 1 } else { 0 };

    store_value(memory, pc+3, value, instruction.mode3, relative_base);

    Some(pc+4)
}

fn change_relative_base(memory : &mut Vec<Num>,
                        pc : usize,
                        instruction : Instruction,
                        relative_base : &mut Num) -> Option<usize> {
    let op1 = get_value(memory, pc+1, instruction.mode1, *relative_base);
    *relative_base = (*relative_base) + op1; 
    Some(pc+2)                        
}

fn read_input(filename : &str) -> Vec<Num> {
    let mut contents = String::new();

    File::open(filename)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    contents.split(',')
            .map(|n| Num::from_str_radix(&n.trim(), 10)
                     .unwrap()
                )
            .collect()    
}

fn print_panels(panels : &HashMap<Pos, Num>) {
    let size : i64 = (panels.len()/3) as i64;
    let side = (size * 2 + 1) as usize;
 
    let mut grid = vec![vec![0; side]; side];

    for entry in panels {
        let pos = entry.0;
        let col = entry.1;

        if *col == 1 {
            let pos_x_in_grid = pos.0 + size;
            let pos_y_in_grid = size - pos.1;

            grid[pos_x_in_grid as usize][pos_y_in_grid as usize] = 1;
        }
    
    }
    
    for i in 0..size*2+1 {
        for j in 0..size*2+1 {
            print!("{}",grid[i as usize][j as usize]);
        }
        println!("");
    }
}

fn count_panels(panels : &HashMap<Pos, Num>) -> usize {
    panels.iter().filter(|(_, &p)| p == 2).count()
}

fn print_grid(grid : [[Num; 40]; 20]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{:?}", grid[i][j]);
        }
        println!();
    }
}

fn main() {
    let mut memory = read_input("input.txt");

    // Set quarters to play free
    memory[0] = 2;

    println!("Welcome to the INTCODE computer!");
    
    let mut pc = 0;
    let mut relative_base = 0;
    let mut out_buffer = Vec::with_capacity(3);

    let mut grid = [[0; 40]; 20];
    let mut score = 0;
    
    while pc < memory.len() {
        let instruction = decode(memory[pc]);
        let new_pc = match instruction.opcode {
            99 => None,
            1 | 2 => fetch_operands_and_store_result(&mut memory, 
                pc, instruction, relative_base),
            3 => {
                let mut buffer = String::new();
    
                println!("Please input a value");
                io::stdin().read_line(&mut buffer).unwrap();
    
                let result = Num::from_str_radix(&buffer.trim(), 10).unwrap();
                input(&mut memory, pc, instruction, relative_base, result)
            },
            4 => {
                let mut out = 0; 
                let new_pc = output(&mut memory, pc, instruction, relative_base, &mut out);
                
                out_buffer.push(out);

                if out_buffer.len() == 3 {
                    if out_buffer[0] == -1 && out_buffer[1] == 0 {
                        score = out_buffer[2];
                    } else {
                        grid[out_buffer[1] as usize][out_buffer[0] as usize] = out_buffer[2];
                    }  
                    out_buffer.clear();

                    // print!("\x1B[2J");
                    println!("Score {:?}", score);
                    print_grid(grid);
                }

                new_pc
            },
            5 | 6 => jump_if(&mut memory, pc, instruction, relative_base),
            7 | 8 => comparison(&mut memory, pc, instruction, 
                relative_base),
            9 => change_relative_base(&mut memory, pc, instruction, 
                &mut relative_base),
            _ => panic!("Unexpected opcode in instruction {:?}", instruction),
        };

        match new_pc {
            Some(new_pc) => {pc = new_pc;}
            None => {
                println!("Halt!"); 
                return;
            }
        }
    }
}

// fn main() {
//     let mut memory = read_input("input.txt");

//     println!("Welcome to the INTCODE computer!");
    
//     let mut pc = 0;
//     let mut relative_base = 0;
//     let mut curr_input;
//     let mut out_buffer = Vec::with_capacity(3);

//     let mut position = (0, 0);
//     let mut direction = Direction::N;

//     let mut panels : HashMap<Pos, Num> = HashMap::new();
//     // panels.insert((0,0), 1);
    
//     while pc < memory.len() {
//         let instruction = decode(memory[pc]);
//         let new_pc = match instruction.opcode {
//             99 => None,
//             1 | 2 => fetch_operands_and_store_result(&mut memory, 
//                 pc, instruction, relative_base),
//             3 => {
//                 curr_input = match panels.get(&position) {
//                     Some(color) => *color,
//                     None => 0,
//                 };
//                 input(&mut memory, pc, instruction, relative_base, curr_input)
//             },
//             4 => {
//                 let mut out = 0; 
//                 let new_pc = output(&mut memory, pc, instruction, relative_base, &mut out);
                
//                 out_buffer.push(out);

//                 if out_buffer.len() == 3 {
//                     panels.insert((out_buffer[0], out_buffer[1]), out_buffer[2]);
//                     out_buffer.clear();
//                 }

//                 new_pc
//             },
//             5 | 6 => jump_if(&mut memory, pc, instruction, relative_base),
//             7 | 8 => comparison(&mut memory, pc, instruction, 
//                 relative_base),
//             9 => change_relative_base(&mut memory, pc, instruction, 
//                 &mut relative_base),
//             _ => panic!("Unexpected opcode in instruction {:?}", instruction),
//         };
//         match new_pc {
//             Some(new_pc) => {pc = new_pc;}
//             None => {
//                 println!("Halt!");
//                 // println!("Panels {:?}", panels);
//                 println!("Number of unique panels {:?}", panels.len());
//                 println!("Number of panels equal to a block {:?}", count_panels(&panels)); 
//                 return;
//             }
//         }
//     }
// }