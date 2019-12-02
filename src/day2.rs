pub fn p1() -> String {
    let input = program_input_generator(12, 2);
    return ship_compute(input.as_str())[0].to_string();
}

pub fn p2() -> String {
    let mut found_noun = 0;
    let mut found_verb = 0;

    for i in 0..=99 {
        for j in 0..=99 {
            let next_input = program_input_generator(i, j);
            let comptued_value = &ship_compute(next_input.as_str())[0];

            if comptued_value == "19690720" {
                found_noun = i;
                found_verb = j;
                println!("found noun: {:?}, verb: {:?}", found_noun, found_verb);
            }
        }
    }

    return (100 * found_noun + found_verb).to_string();
}

fn program_input_generator(noun: i32, verb: i32) -> String {
    return format!("1,{},{},3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,19,5,23,1,23,5,27,2,27,10,31,1,31,9,35,1,35,5,39,1,6,39,43,2,9,43,47,1,5,47,51,2,6,51,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,2,10,71,75,1,6,75,79,2,79,9,83,1,83,5,87,1,87,9,91,1,91,9,95,1,10,95,99,1,99,13,103,2,6,103,107,1,107,5,111,1,6,111,115,1,9,115,119,1,119,9,123,2,123,10,127,1,6,127,131,2,131,13,135,1,13,135,139,1,9,139,143,1,9,143,147,1,147,13,151,1,151,9,155,1,155,13,159,1,6,159,163,1,13,163,167,1,2,167,171,1,171,13,0,99,2,0,14,0", noun, verb);
}

fn ship_compute(program: &str) -> Vec<String> {
    let mut current_position = 0;
    let mut program_instructions = program
        .split(",")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_1() {
        let input = "1,0,0,0,99";
        let calculated_output = ship_compute(input).join(",");
        let expected_output = "2,0,0,0,99";
        assert_eq!(expected_output, calculated_output);
    }

    #[test]
    fn test_computer_2() {
        let input = "2,3,0,3,99";
        let calculated_output = ship_compute(input).join(",");
        let expected_output = "2,3,0,6,99";
        assert_eq!(expected_output, calculated_output);
    }

    #[test]
    fn test_computer_3() {
        let input = "2,4,4,5,99,0";
        let calculated_output = ship_compute(input).join(",");
        let expected_output = "2,4,4,5,99,9801";
        assert_eq!(expected_output, calculated_output);
    }

    #[test]
    fn test_computer_4() {
        let input = "1,1,1,4,99,5,6,0,99";
        let calculated_output = ship_compute(input).join(",");
        let expected_output = "30,1,1,4,2,5,6,0,99";
        assert_eq!(expected_output, calculated_output);
    }

    #[test]
    fn test_computer_d2p1() {
        let input = program_input_generator(12, 2);
        let calculated_output = &ship_compute(input.as_str())[0];
        let expected_output = "3654868";
        assert_eq!(expected_output, calculated_output);
    }
}
