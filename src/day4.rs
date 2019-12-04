use itertools::Itertools;

const RANGE_START: i32 = 165_432;
const RANGE_END: i32 = 707_912;

pub fn p1() -> String {
    (RANGE_START..RANGE_END)
        .map(|i| i.to_string())
        .map(|i| valid_password_p1(i, false))
        .filter(|i| *i)
        .count()
        .to_string()
}

pub fn p2() -> String {
    (RANGE_START..RANGE_END)
        .map(|i| i.to_string())
        .map(|i| valid_password_p2(i, false))
        .filter(|i| *i)
        .count()
        .to_string()
}

fn valid_password_p1(password: String, testing: bool) -> bool {
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

        if decreasing(&password, i) {
            return false
        }

        if matches_last(&password, i) {
            double_numbers = true;
        }
    }

    if !double_numbers {
        return false
    }

    true
}

fn valid_password_p2(password: String, testing: bool) -> bool {
    let letter_count = password.chars().count();
    let numeric_password: i32 = password.parse().unwrap();
    if letter_count != 6 {
        return false
    }

    if outside_range(numeric_password) && !testing {
        return false
    }

    let multiples = more_than_two(&password);
    let double_numbers = exactly_two(&password);
    for i in 0..letter_count {
        if i == 0 {
            continue
        }

        if decreasing(&password, i) {
            return false
        }
    }

    if !double_numbers {
        return false
    }

    if !double_numbers && multiples {
        return false
    }

    // double_numbers == true
    // multples == false

    true
}

fn decreasing(password: &str, i: usize) -> bool {
    if i < 1 {
        return true
    }
    password.chars().nth(i) < password.chars().nth(i-1)
}

fn matches_last(password: &str, i: usize) -> bool {
    if i < 1 {
        return true
    }
    password.chars().nth(i) == password.chars().nth(i-1)
}

fn outside_range(password: i32) -> bool {
    password > RANGE_END || password < RANGE_START
}

fn more_than_two(password: &str) -> bool {
    password
        .chars()
        .map(|c| (c, 1))
        .coalesce(|(c, n), (d, m)|
            if c == d { Ok((c, n + m)) } else { Err(((c, n), (d, m))) }
        )
        .filter(|(_, n)| n > &2 )
        .map(|(c, _)| c)
        .count() > 0
}

fn exactly_two(password: &str) -> bool {
    password
        .chars()
        .map(|c| (c, 1))
        .coalesce(|(c, n), (d, m)|
            if c == d { Ok((c, n + m)) } else { Err(((c, n), (d, m))) }
        )
        .filter(|(_, n)| n == &2)
        .map(|(c, _)| c)
        .count() >= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_all_ones() {
        let password = "111111".to_string();
        assert_eq!(true, valid_password_p1(password, true));
    }

    #[test]
    fn test_password_descreasing() {
        let password = "223450".to_string();
        assert_eq!(false, valid_password_p1(password, true));
    }

    #[test]
    fn test_password_no_double() {
        let password = "123789".to_string();
        assert_eq!(false, valid_password_p1(password, true));
    }

    #[test]
    fn test_password_p2_correct() {
        let password = "112233".to_string();
        assert_eq!(true, valid_password_p2(password, true));
    }

    #[test]
    fn test_password_p2_triplets() {
        let password = "123444".to_string();
        assert_eq!(false, valid_password_p2(password, true));
    }

    #[test]
    fn test_password_p2_quad_1_double_2() {
        let password = "111122".to_string();
        assert_eq!(true, valid_password_p2(password, true));
    }
}
