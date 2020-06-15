use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> Option<isize> {
    if let Ok(lines) = read_lines(filename) {
        return Some(calculate_frequency(lines))
    }

    None
}

fn calculate_frequency<T>(lines: T) -> isize
where T: Iterator<Item = Result<String, io::Error>>
{
    let mut count = 0;

    for line in lines {
        if let Some(n) = line.ok().and_then(parse_frequency_change) {
            count += n;
        }
    }

    count
}

fn parse_frequency_change(line: String) -> Option<isize> {
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
        let actual = parse_frequency_change("+5".to_string());
        
        assert_eq!(Some(5), actual);
    }

    #[test]
    fn succeeds_parsing_negative_frequency_change() {
        let actual = parse_frequency_change("-5".to_string());
        
        assert_eq!(Some(-5), actual);
    }

    #[test]
    fn calculates_frequency_positive_numbers() {
        let actual = 
            calculate_frequency(
                vec![
                    Ok("+1".to_string()), 
                    Ok("+3".to_string()), 
                    Ok("+6".to_string())
                ].into_iter());

        assert_eq!(10, actual);
    }
}
