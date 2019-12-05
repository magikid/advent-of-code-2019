pub fn compute(program: &str) -> Vec<String> {
    let mut current_position = 0;
    let mut program_instructions = program
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    loop {
        let next_instruction: &i32 = &program_instructions[current_position];

        match next_instruction {
            1 => {
                let first_argument_position: usize =
                    program_instructions[current_position + 1] as usize;
                let first_argument = program_instructions[first_argument_position];
                let second_argument_position: usize =
                    program_instructions[current_position + 2] as usize;
                let second_argument = program_instructions[second_argument_position];
                let output_placement: usize = program_instructions[current_position + 3] as usize;
                let operation_output = first_argument + second_argument;
                program_instructions[output_placement] = operation_output;
                current_position += 4;
            }
            2 => {
                let first_argument_position: usize =
                    program_instructions[current_position + 1] as usize;
                let first_argument = program_instructions[first_argument_position];
                let second_argument_position: usize =
                    program_instructions[current_position + 2] as usize;
                let second_argument = program_instructions[second_argument_position];
                let output_placement: usize = program_instructions[current_position + 3] as usize;
                let multiply_output = first_argument * second_argument;
                program_instructions[output_placement] = multiply_output;
                current_position += 4;
            }
            99 => {
                return program_instructions
                    .iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<_>>();
            }
            _ => panic!("opcode not defined"),
        }

        if current_position > program_instructions.len() {
            panic!("how did we get here?");
        }
    }
}
