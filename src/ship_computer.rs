#[derive(Debug)]
struct Program {
    instructions: Vec<i32>,
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

    fn get_output(&self) -> String {
        self.output.clone()
    }

    fn from(instructions: Vec<i32>) -> Program {
        Program{instructions, position: 0, output: String::from("")}
    }
}

pub fn compute(program: &str) -> Program {
    let program_instructions = program
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();
    let mut program = Program::from(program_instructions);
    let mut output = String::from("");
    trace!("initial program: {:?}", program);

    loop {
        let next_instruction: &i32 = &program.instructions[program.position];

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
                debug!("Writing input {:?} to location {:?}", input, output_placement);
                program.advance_position(2);
            }

            4 => {
                let value_to_output = get_argument(&program, 1);
                output.push_str(format!("output {:?}\n", value_to_output).as_str());
                program.advance_position(2);
            }

            99 => {
                info!("{}", output);
                return program.instructions
                    .iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<_>>();
            }

            _ => panic!("opcode not defined"),
        }

        if program.position > program.instructions.len() {
            panic!("how did we get here?");
        }
    }
}

fn get_argument(program: &Program, argument_offset: usize) -> i32 {
    let argument_position: usize = program.instructions[program.position + argument_offset] as usize;
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
