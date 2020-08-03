use super::ship_computer;

pub fn p1() -> String {
    "p1".to_string()
}

pub fn p2() -> String {
    "p2".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder()
            .filter(None, log::LevelFilter::Trace)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_computer_output() {
        init();
        let instructions = "4,2,99";
        let computer = ship_computer::compute(instructions);
        let computed_output = computer.get_output();
        assert_eq!("output 99\n", computed_output);
    }

    #[test]
    fn test_computer_immediate_mode() {
        init();
        let instructions = "1002,4,3,4,33";
        let expected_output = ship_computer::compute(instructions).join(",");
        assert_eq!("99", expected_output);
    }
}
