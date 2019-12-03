use std::collections::HashSet;

const D3P1_WIRE_1: &str = "R1004,U518,R309,D991,R436,D360,L322,U627,R94,D636,L846,D385,R563,U220,L312,D605,L612,D843,R848,U193,L671,D852,L129,D680,L946,D261,L804,D482,R196,U960,L234,U577,R206,D973,R407,D400,R44,D103,R463,U907,L972,U628,L962,U856,L564,D25,L425,U332,R931,U837,R556,U435,R88,U860,L982,D393,R793,D86,R647,D337,R514,D361,L777,U640,R833,D674,L817,D260,R382,U168,R161,U449,L670,U814,L42,U461,R570,U855,L111,U734,L699,U602,R628,D79,L982,D494,L616,D484,R259,U429,L917,D321,R429,U854,R735,D373,L508,D59,L207,D192,L120,D943,R648,U245,L670,D571,L46,D195,L989,U589,L34,D177,L682,U468,L783,D143,L940,U412,R875,D604,R867,D951,L82,U851,L550,D21,L425,D81,L659,D231,R92,D232,R27,D269,L351,D369,R622,U737,R531,U693,R295,U217,R249,U994,R635,U267,L863,U690,L398,U576,R982,U252,L649,U321,L814,U516,R827,U74,L80,U624,L802,D620,L544,U249,R983,U424,R564,D217,R151,U8,L813,D311,R203,U478,R999,U495,R957,U641,R40,U431,L830,U67,L31,U532,R345,U878,L996,D223,L76,D264,R823,U27,L776,U936,L614,U421,L398,U168,L90,U525,R640,U95,L761,U938,R296,D463,L349,D709,R428,U818,L376,D444,L748,D527,L755,U750,R175,U495,R587,D767,L332,U665,L84,D747,L183,D969,R37,D514,R949,U985,R548,U939,L170,U415,R857,D480,R836,D363,R763,D997,R721,D140,R699,U673,L724,U375,R55,U758,R634,D590,L608,U674,R809,U308,L681,D957,R30,D913,L633,D939,L474,D567,R290,D615,L646,D478,L822,D471,L952,D937,R306,U380,R695,U788,R555,D64,R769,D785,R115,U474,R232,U353,R534,D268,L434,U790,L777,D223,L168,U21,L411,D524,R862,D43,L979,U65,R771,U872,L983,U765,R162";
const D3P1_WIRE_2: &str = "L998,U952,R204,U266,R353,U227,L209,D718,L28,D989,R535,U517,L934,D711,R878,U268,L895,D766,L423,U543,L636,D808,L176,U493,R22,D222,R956,U347,R953,U468,R657,D907,R464,U875,L162,U225,L410,U704,R76,D985,L711,U176,R496,D720,L395,U907,R223,D144,R292,D523,R514,D942,R838,U551,L487,D518,L159,D880,R53,D519,L173,D449,R525,U645,L65,D568,R327,U667,R790,U131,R402,U869,R287,D411,R576,D265,R639,D783,R629,U107,L571,D247,L61,D548,L916,D397,R715,U138,R399,D159,L523,U2,R794,U699,R854,U731,L234,D135,L98,U702,L179,D364,R123,D900,L548,U880,R560,D648,L701,D928,R256,D970,L396,U201,L47,U156,R723,D759,R663,D306,L436,U508,R371,D494,L147,U131,R946,D207,L516,U514,R992,D592,L356,D869,L299,U10,R744,D13,L52,U749,R400,D146,L193,U720,L226,U973,R971,U691,R657,D604,L984,U652,L378,D811,L325,D714,R131,D428,R418,U750,L706,D855,L947,U557,L985,D688,L615,D114,R202,D746,R987,U353,R268,U14,R709,U595,R982,U332,R84,D620,L75,D885,L269,D544,L137,U124,R361,U502,L290,D710,L108,D254,R278,U47,R74,U293,R237,U83,L80,U661,R550,U886,L201,D527,L351,U668,R366,D384,L937,D768,L906,D388,L604,U515,R632,D486,L404,D980,L652,U404,L224,U957,L197,D496,R690,U407,L448,U953,R391,U446,L964,U372,R351,D786,L187,D643,L911,D557,R254,D135,L150,U833,R876,U114,R688,D654,L991,U717,R649,U464,R551,U886,L780,U293,L656,U681,L532,U184,L903,D42,L417,D917,L8,U910,L600,D872,L632,D221,R980,U438,R183,D973,L321,D652,L540,D163,R796,U404,L507,D495,R707,U322,R16,U59,L421,D255,L463,U462,L524,D703,L702,D904,L597,D385,L374,U411,L702,U804,R706,D56,L288";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MovementInstruction {
    direction: Direction,
    steps: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct PathPoint {
    y: i32,
    x: i32,
}

impl PathPoint {
    fn manhattan_distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub fn p1() -> String {
    closest_intersection(D3P1_WIRE_1, D3P1_WIRE_2).to_string()
}

pub fn p2() -> String {
    "p2".to_string()
}

fn closest_intersection(first_wire_path: &str, second_wire_path: &str) -> i32 {
    let first_wire_points = wire_points(first_wire_path);
    let second_wire_points = wire_points(second_wire_path);
    let intersections = wire_intersections(first_wire_points, second_wire_points);
    let mut shortest_distance = 1_000_000;

    for intersection in intersections {
        let current_distance = intersection.manhattan_distance_from_origin();
        if current_distance < shortest_distance {
            shortest_distance = current_distance;
        }
        println!("intersection: {:?}, current_distance: {:?}, shortest_distance: {:?}", intersection, current_distance, shortest_distance);
    }

    shortest_distance
}

fn wire_intersections(first_wire_points: Vec<PathPoint>, second_wire_points: Vec<PathPoint>) -> Vec<PathPoint> {
    let first_wire_set: HashSet<PathPoint> = first_wire_points.iter().cloned().collect();
    let second_wire_set: HashSet<PathPoint> = second_wire_points.iter().cloned().collect();

    first_wire_set.intersection(&second_wire_set).cloned().collect::<Vec<PathPoint>>()
}

fn wire_points(wire_path: &str) -> Vec<PathPoint> {
    let instructions = parse_instructions(wire_path);
    let mut all_points = Vec::with_capacity(instructions.len());

    let mut current_x = 0;
    let mut current_y = 0;

    for current_instruction in instructions.iter() {
        for _ in 1..=current_instruction.steps {
            let mut dy = 0;
            let mut dx = 0;

            match current_instruction.direction {
                Direction::Up => dy = 1,
                Direction::Down => dy = -1,
                Direction::Right => dx = 1,
                Direction::Left => dx = -1,
            }

            current_x += dx;
            current_y += dy;

            all_points.push(PathPoint {
                x: current_x,
                y: current_y,
            });
        }
    }

    all_points
}

fn parse_instructions(wire_path: &str) -> Vec<MovementInstruction> {
    wire_path
        .split(',')
        .map(|instruction| {
            let direction = parse_direction(instruction);
            let steps = parse_steps(instruction);

            MovementInstruction { direction, steps }
        })
        .collect()
}

fn parse_direction(raw_instruction: &str) -> Direction {
    match raw_instruction.chars().nth(0).unwrap() {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction found {:?}", raw_instruction),
    }
}

fn parse_steps(raw_instruction: &str) -> i32 {
    raw_instruction
        .chars()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap()
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

    #[test]
    fn test_parse_direction_u() {
        let instruction = "U1102";
        assert_eq!(Direction::Up, parse_direction(instruction));
    }

    #[test]
    fn test_parse_direction_d() {
        let instruction = "D1102";
        assert_eq!(Direction::Down, parse_direction(instruction));
    }

    #[test]
    fn test_parse_direction_r() {
        let instruction = "R1102";
        assert_eq!(Direction::Right, parse_direction(instruction));
    }

    #[test]
    fn test_parse_direction_l() {
        let instruction = "L1102";
        assert_eq!(Direction::Left, parse_direction(instruction));
    }

    #[test]
    fn test_parse_steps_low() {
        let instruction = "U1";
        assert_eq!(1, parse_steps(instruction));
    }

    #[test]
    fn test_parse_steps_high() {
        let instruction = "U1102";
        assert_eq!(1102, parse_steps(instruction));
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = "U1,R2";
        let expected_output = vec![
            MovementInstruction {
                direction: Direction::Up,
                steps: 1,
            },
            MovementInstruction {
                direction: Direction::Right,
                steps: 2,
            },
        ];
        assert_eq!(expected_output, parse_instructions(instructions));
    }

    #[test]
    fn test_wire_points() {
        let instructions = "U1,R2";
        let expected_output = vec![PathPoint{x: 0, y: 1}, PathPoint{x: 1, y: 1}, PathPoint{x: 2, y: 1}];
        assert_eq!(expected_output, wire_points(instructions));
    }

    #[test]
    fn test_wire_intersections() {
        let first_wire_points = wire_points("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let second_wire_points = wire_points("U62,R66,U55,R34,D71,R55,D58,R83");
        let mut expected_output = vec![PathPoint{x: 146, y: 46}, PathPoint{x: 158, y: -12}, PathPoint{x: 155, y: 11}, PathPoint{x: 155, y: 4}];
        expected_output.sort();
        let mut calculated_output = wire_intersections(first_wire_points, second_wire_points);
        calculated_output.sort();
        assert_eq!(expected_output, calculated_output);
    }
}