use std::collections::HashSet;

pub mod part_1 {
    use std::fs;
    use super::calculate_frequency;

    pub fn run(filename: &str) -> Option<isize> {
        if let Ok(contents) = fs::read_to_string(filename) {
            let lines: Vec<&str> = contents.lines().collect();
            return Some(calculate_frequency(lines))
        }
    
        None
    }
}

pub mod part_2 {
    use std::fs;
    use super::calibrate_device;

    pub fn run(filename: &str) -> Option<isize> {
        if let Ok(contents) = fs::read_to_string(filename) {
            let lines: Vec<&str> = contents.lines().collect();
            return Some(calibrate_device(lines))
        }

        None
    }
}

fn calibrate_device<'a>(lines: Vec<&'a str>) -> isize {
    let mut frequencies: HashSet<isize> = HashSet::new();
    let mut current = 0;

    frequencies.insert(current);

    loop {
        for line in &lines {
            if let Some(n) = parse_frequency_change(line) {
                current += n;

                if !frequencies.insert(current) {
                    return current
                }

            }
        }
    }
}

fn calculate_frequency<'a>(lines: Vec<&'a str>) -> isize {
    let mut current = 0;

    for line in lines {
        if let Some(n) = parse_frequency_change(&line[..]) {
            current += n;
        }
    }

    current
}

fn parse_frequency_change(line: &str) -> Option<isize> {
    line.parse::<isize>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeeds_parsing_positive_frequency_change() {
        let actual = parse_frequency_change("+5");
        
        assert_eq!(Some(5), actual);
    }

    #[test]
    fn succeeds_parsing_negative_frequency_change() {
        let actual = parse_frequency_change("-5");
        
        assert_eq!(Some(-5), actual);
    }

    #[test]
    fn calculates_frequency_returns_3() {
        let actual = calculate_frequency(vec![ "+1", "+3", "-6", "+10", "-5"]);

        assert_eq!(3, actual);
    }

    #[test]
    fn calibrates_device_returns_0() {
        let actual = calibrate_device(vec!["+1", "-1", ]);

        assert_eq!(0, actual);
    }

    #[test]
    fn calibrates_device_returns_10() {
        let actual = calibrate_device(vec!["+3", "+3", "+4","-2", "-4"]);

        assert_eq!(10, actual);
    }

    #[test]
    fn calibrates_device_returns_5() {
        let actual = calibrate_device(vec!["-6", "+3", "+8", "+5", "-6"]);

        assert_eq!(5, actual);
    }

    #[test]
    fn calibrates_device_returns_14() {
        let actual = calibrate_device(vec!["+7", "+7", "-2", "-7", "-4"]);

        assert_eq!(14, actual);
    }
}
