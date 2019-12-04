const RANGE_START: i32 = 165_432;
const RANGE_END: i32 = 707_912;

pub fn p1() -> String {
    (RANGE_START..RANGE_END)
        .map(|i| i.to_string())
        .map(|i| valid_password(i, false))
        .filter(|i| *i)
        .count()
        .to_string()
}

pub fn p2() -> &'static str {
    "p2"
}

fn valid_password(password: String, testing: bool) -> bool {
    let letter_count = password.chars().count();
    let numeric_password: i32 = password.parse().unwrap();
    if letter_count != 6 {
        return false
    }

    if outside_range(numeric_password) && !testing {
        return false
    }

    let mut double_numbers = false;
    for i in 0..letter_count {
        if i == 0 {
            continue
        }

        if password.chars().nth(i) < password.chars().nth(i-1) {
            return false
        }

        if password.chars().nth(i) == password.chars().nth(i-1) {
            double_numbers = true;
        }
    }

    if !double_numbers {
        return false
    }

    true
}

fn outside_range(password: i32) -> bool {
    password > RANGE_END || password < RANGE_START
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_all_ones() {
        let password = "111111";
        assert_eq!(true, valid_password(password, true));
    }

    #[test]
    fn test_password_descreasing() {
        let password = "223450";
        assert_eq!(false, valid_password(password, true));
    }

    #[test]
    fn test_password_no_double() {
        let password = "123789";
        assert_eq!(false, valid_password(password, true));
    }

}
