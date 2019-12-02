use std::fs;

pub fn p1() -> i32 {
    let final_weight = get_weights()
        .into_iter()
        .map(|weight| fuel_requirement(weight as f32))
        .sum();

    return final_weight;
}

pub fn p2() -> i32 {
    let initial_weights = get_weights();
    let mut final_fuel_weights: Vec<i32> = Vec::new();

    for weight in initial_weights {
        let fuel_weight = total_fuel_requirement(weight as f32);
        final_fuel_weights.push(fuel_weight);
    }

    return final_fuel_weights.iter().sum();
}

fn get_weights<'a>() -> Vec<i32> {
    return fs::read_to_string("inputs/d1p1.txt")
        .expect("Something went wrong reading the file")
        .split_whitespace()
        .map(|s| parse_num(s))
        .collect::<Vec<_>>();
}

fn fuel_requirement(weight: f32) -> i32 {
    return (weight / 3.0).floor() as i32 - 2;
}

fn total_fuel_requirement(weight: f32) -> i32 {
    let mut prev_fuel = fuel_requirement(weight);
    let mut fuel_weights = Vec::new();
    let mut new_fuel = 0;

    while new_fuel >= 0 {
        fuel_weights.push(prev_fuel);
        new_fuel = fuel_requirement(prev_fuel as f32);
        prev_fuel = new_fuel;
    }

    return fuel_weights.iter().sum();
}

fn parse_num(l: &str) -> i32 {
    return l.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fuel_requirement() {
        let weight = 12.0;
        let fuel = fuel_requirement(weight);
        assert_eq!(2, fuel);
    }

    #[test]
    fn test_odd_fuel_requirement() {
        let weight = 14.0;
        let fuel = fuel_requirement(weight);
        assert_eq!(2, fuel);
    }

    #[test]
    fn test_big_fuel_requirement() {
        let weight = 1969.0;
        let fuel = fuel_requirement(weight);
        assert_eq!(654, fuel);
    }

    #[test]
    fn test_bigger_fuel_requirement() {
        let weight = 100756.0;
        let fuel = fuel_requirement(weight);
        assert_eq!(33583, fuel);
    }

    #[test]
    fn test_total_fuel_requirement() {
        let weight = 14.0;
        let fuel = total_fuel_requirement(weight);
        assert_eq!(2, fuel);
    }

    #[test]
    fn test_more_total_fuel_requirement() {
        let weight = 1969.0;
        let fuel = total_fuel_requirement(weight);
        assert_eq!(966, fuel);
    }

    #[test]
    fn test_bigger_total_fuel_requirement() {
        let weight = 100756.0;
        let fuel = total_fuel_requirement(weight);
        assert_eq!(50346, fuel);
    }
}