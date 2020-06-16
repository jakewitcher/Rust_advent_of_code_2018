use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub mod part_1 {
    use super::{read_lines, calculate_frequency};

    pub fn run(filename: &str) -> Option<isize> {
        if let Ok(lines) = read_lines(filename) {
            return Some(calculate_frequency(lines))
        }
    
        None
    }
}

pub mod part_2 {
    use super::{read_lines, calibrate_device};

    pub fn run(filename: &str) -> Option<isize> {
        if let Ok(lines) = read_lines(filename) {
            return Some(calibrate_device(lines))
        }

        None
    }
}

fn calibrate_device<T>(lines: T) -> isize
    where T: Iterator<Item = Result<String, std::io::Error>>
    {
        let mut frequencies: HashSet<isize> = HashSet::new();
        let mut current = 0;
    
        frequencies.insert(current);
    
        let lines: Vec<String> = 
            lines.filter_map(
                |line| line.ok()).collect();
    
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

fn calculate_frequency<T>(lines: T) -> isize
    where T: Iterator<Item = Result<String, io::Error>>
    {
        let mut current = 0;

        for line in lines {
            if let Some(n) = line.ok().and_then(|line| parse_frequency_change(&line[..])) {
                current += n;
            }
        }

        current
    }

fn parse_frequency_change(line: &str) -> Option<isize> {
    line.parse::<isize>().ok()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    match File::open(filename) {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(err) => {
            println!("{}", err);
            Err(err)
        }
    }
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
        let actual = 
            calculate_frequency(
                vec![
                    Ok("+1".to_string()), 
                    Ok("+3".to_string()), 
                    Ok("-6".to_string()),
                    Ok("+10".to_string()), 
                    Ok("-5".to_string())
                ].into_iter());

        assert_eq!(3, actual);
    }

    #[test]
    fn calibrates_device_returns_0() {
        let actual = 
            calibrate_device(
                vec![
                    Ok("+1".to_string()), 
                    Ok("-1".to_string()), 
                ].into_iter());

        assert_eq!(0, actual);
    }

    #[test]
    fn calibrates_device_returns_10() {
        let actual = 
            calibrate_device(
                vec![
                    Ok("+3".to_string()), 
                    Ok("+3".to_string()), 
                    Ok("+4".to_string()),
                    Ok("-2".to_string()), 
                    Ok("-4".to_string())
                ].into_iter());

        assert_eq!(10, actual);
    }

    #[test]
    fn calibrates_device_returns_5() {
        let actual = 
            calibrate_device(
                vec![
                    Ok("-6".to_string()), 
                    Ok("+3".to_string()), 
                    Ok("+8".to_string()),
                    Ok("+5".to_string()), 
                    Ok("-6".to_string())
                ].into_iter());

        assert_eq!(5, actual);
    }

    #[test]
    fn calibrates_device_returns_14() {
        let actual = 
            calibrate_device(
                vec![
                    Ok("+7".to_string()), 
                    Ok("+7".to_string()), 
                    Ok("-2".to_string()),
                    Ok("-7".to_string()), 
                    Ok("-4".to_string())
                ].into_iter());

        assert_eq!(14, actual);
    }
}
