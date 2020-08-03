#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<i32>,
    position: usize,
    output: String,
}

impl Program {
    fn advance_position(&mut self, steps: usize) {
        self.position += steps
    }

    fn print_output(&self) {
        println!("{}", self.output)
    }

    pub fn get_output(&self) -> &str {
        self.output.as_str()
    }

    fn from(instructions: Vec<i32>) -> Program {
        Program {
            instructions,
            position: 0,
            output: String::from(""),
        }
    }

    pub fn join(&self, joiner: &str) -> String {
        self.instructions
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(joiner)
    }

    fn next_instruction(&self) -> i32 {
        let combined_opcode_and_parameter_modes = self.instructions[self.position];
        let combined_length = combined_opcode_and_parameter_modes
            .to_string()
            .chars()
            .collect::<Vec<_>>()
            .len();
    }
}

pub fn compute(program: &str) -> Program {
    let program_instructions = program
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();
    let mut program = Program::from(program_instructions);
    trace!("initial program: {:?}", program);

    loop {
        let next_instruction: i32 = program.next_instruction();

        match next_instruction {
            1 => {
                let first_argument = get_argument(&program, 1);
                let second_argument = get_argument(&program, 2);
                let output_placement: usize = program.instructions[program.position + 3] as usize;
                let operation_output = first_argument + second_argument;
                program.instructions[output_placement] = operation_output;
                program.advance_position(4);
            }

            2 => {
                let first_argument = get_argument(&program, 1);
                let second_argument = get_argument(&program, 2);
                let output_placement: usize = program.instructions[program.position + 3] as usize;
                let multiply_output = first_argument * second_argument;
                program.instructions[output_placement] = multiply_output;
                program.advance_position(4);
            }

            3 => {
                let output_placement = get_argument(&program, 1);
                let input = get_input();
                debug!(
                    "Writing input {:?} to location {:?}",
                    input, output_placement
                );
                program.advance_position(2);
            }

            4 => {
                let value_to_output = get_argument(&program, 1);
                program
                    .output
                    .push_str(format!("output {:?}\n", value_to_output).as_str());
                program.print_output();
                program.advance_position(2);
            }

            99 => {
                info!("{}", program.output);
                return program;
            }

            _ => panic!("opcode not defined"),
        }

        if program.position > program.instructions.len() {
            panic!("how did we get here?");
        }
    }
}

fn get_argument(program: &Program, argument_offset: usize) -> i32 {
    let argument_position: usize =
        program.instructions[program.position + argument_offset] as usize;
    if argument_position > program.instructions.len() {
        panic!("oops");
    }
    program.instructions[argument_position]
}

fn get_input() -> String {
    let mut program_input = String::new();
    std::io::stdin().read_line(&mut program_input).unwrap();
    program_input
}
