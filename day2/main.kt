var memory = mutableListOf(1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,2,9,19,23,1,23,6,27,1,13,27,31,1,31,10,35,1,9,35,39,1,39,9,43,2,6,43,47,1,47,5,51,2,10,51,55,1,6,55,59,2,13,59,63,2,13,63,67,1,6,67,71,1,71,5,75,2,75,6,79,1,5,79,83,1,83,6,87,2,10,87,91,1,9,91,95,1,6,95,99,1,99,6,103,2,103,9,107,2,107,10,111,1,5,111,115,1,115,6,119,2,6,119,123,1,10,123,127,1,127,5,131,1,131,2,135,1,135,5,0,99,2,0,14,0);
//var memory = mutableListOf(1,1,1,4,99,5,6,0,99);

fun main(args: Array<String>) {
    // memory[1] = 12;
    // memory[2] = 2;

    for (noun in 0..99) {
        for (verb in 0..99) {
            memory = mutableListOf(1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,2,9,19,23,1,23,6,27,1,13,27,31,1,31,10,35,1,9,35,39,1,39,9,43,2,6,43,47,1,47,5,51,2,10,51,55,1,6,55,59,2,13,59,63,2,13,63,67,1,6,67,71,1,71,5,75,2,75,6,79,1,5,79,83,1,83,6,87,2,10,87,91,1,9,91,95,1,6,95,99,1,99,6,103,2,103,9,107,2,107,10,111,1,5,111,115,1,115,6,119,2,6,119,123,1,10,123,127,1,127,5,131,1,131,2,135,1,135,5,0,99,2,0,14,0);
            memory[1] = noun;
            memory[2] = verb;
            for (i in 0..(memory.size-1) step 4) {
                var opcode = memory[i];

                if (opcode == 99) {
                    println("halt! value at position 0 is ${memory[0]}");
                    if (memory[0] == 19690720) {
                        println("noun ${noun} and verb ${verb}");
                        return;
                    }
                    break;
                } else if (opcode == 1) {
                    var firstOp = memory[memory[i+1]];
                    var secondOp = memory[memory[i+2]];

                    var result = firstOp + secondOp;

                    // println("sum ${firstOp} with ${secondOp}");

                    memory[memory[i+3]] = result; 
                } else if (opcode == 2) {
                    var firstOp = memory[memory[i+1]];
                    var secondOp = memory[memory[i+2]];

                    var result = firstOp * secondOp;

                    // println("mul ${firstOp} with ${secondOp}");

                    memory[memory[i+3]] = result;
                } else {
                    println("unknown opcode");
                }
            }
        }
    }    
}