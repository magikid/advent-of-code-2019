pub fn p1() -> String {
    "p1".to_string()
}

pub fn p2() -> String {
    "p2".to_string()
}

fn closest_intersection(first_wire_path: &str, second_wire_path: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wires_1() {
        let first_wire_path = "R8,U5,L5,D3";
        let second_wire_path = "U7,R6,D4,L4";
        let calculated_output = closest_intersection(first_wire_path, second_wire_path);
        let expected_output = 6;
        assert_eq!(expected_output, calculated_output);
    }


    #[test]
    fn test_wires_2() {
        let first_wire_path = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let second_wire_path = "U62,R66,U55,R34,D71,R55,D58,R83";
        let calculated_output = closest_intersection(first_wire_path, second_wire_path);
        let expected_output = 159;
        assert_eq!(expected_output, calculated_output);
    }


    #[test]
    fn test_wires_3() {
        let first_wire_path = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let second_wire_path = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let calculated_output = closest_intersection(first_wire_path, second_wire_path);
        let expected_output = 135;
        assert_eq!(expected_output, calculated_output);
    }
}
