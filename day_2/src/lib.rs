use std::collections::HashMap;

pub mod part_1 {
    use std::fs;
    use super::Checksum;

    pub fn run(filename: &str) -> Option<usize> {
        if let Ok(contents) = fs::read_to_string(filename) {
            let lines: Vec<&str> = contents.lines().collect();
            return Some(Checksum::new().calc(lines))
        }
    
        None
    }
}

pub mod part_2 {
    use std::fs;
    use super::compare_ids;

    pub fn run(filename: &str) -> Option<String> {
        if let Ok(contents) = fs::read_to_string(filename) {
            let lines = contents.lines().collect();
            return Some(compare_ids(lines))
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct Checksum {
    twice: usize,
    thrice: usize,
}

impl Checksum {
    fn new() -> Checksum {
        Checksum { twice: 0, thrice: 0 }
    }

    fn calc(&mut self, lines: Vec<&str>) -> usize {
        for line in &lines {
            self.inc(line);
        }

        self.get()
    }

    fn get(&self) -> usize {
        self.twice * self.thrice
    }

    fn inc(&mut self, line: &str) {
        let appearances = Checksum::get_appearances(line);
        let (appears_twice, appears_thrice) = Checksum::get_frequency(&appearances);
    
        if appears_twice {
            self.twice += 1;
        }
    
        if appears_thrice {
            self.thrice += 1;
        }
    }

    fn get_appearances(line: &str) -> HashMap<char, usize> {
        let mut appearances: HashMap<char, usize> = HashMap::new();
        
        for c in line.chars() {
            *appearances.entry(c).or_insert(0) += 1;
        }
    
        appearances 
    }

    fn get_frequency(appearances: &HashMap<char, usize>) -> (bool, bool) {
        let mut appears_twice = false;
        let mut appears_thrice = false;
    
        for val in appearances.values() {
            if val == &2 {
                appears_twice = true;
            } else if val == &3 {
                appears_thrice = true;
            }
        }

        (appears_twice, appears_thrice)
    }
}

fn compare_ids(ids: Vec<&str>) -> String {
    let mut i = 0;

    for source in &ids {
        for target in &ids[i..] {
            if let Some(result) = try_find_matching(target, source) {
                return result
            }
        }

        i += 1;
    }

    String::new()
}

fn try_find_matching(left: &str, right: &str) -> Option<String> {
    let mut common = String::new();
    let mut variance = 0;

    for (a, b) in left.chars().zip(right.chars()) {
        if a == b {
            common.push(a);
        } else {
            variance += 1;
        }
    }

    match variance {
        1 => Some(common),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_three_of_a_and_two_of_b() {
        let appearances = Checksum::get_appearances("abaab");

        assert_eq!(Some(&3), appearances.get(&'a'));
        assert_eq!(Some(&2), appearances.get(&'b'));
    }

    #[test]
    fn increments_checksum() {
        let mut actual = Checksum::new();
        actual.inc("abaab");

        assert_eq!(1, actual.get());
    }

    #[test]
    fn does_not_increments_checksum() {
        let mut actual = Checksum::new();
        actual.inc("abaaa");

        assert_eq!(0, actual.twice);
        assert_eq!(0, actual.thrice);
    }

    #[test]
    fn increments_checksum_only_once() {
        let mut actual = Checksum::new();
        actual.inc("adbacabcdd");

        assert_eq!(1, actual.get());
    }

    #[test]
    fn calculates_checksum() {
        let test_case = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        let actual = Checksum::new().calc(test_case);

        assert_eq!(12, actual);
    }

    #[test]
    fn returns_common_chars_as_string_if_chars_vary_by_one() {
        let actual = try_find_matching("abc", "adc");

        assert_eq!(Some("ac".to_string()), actual);
    }

    #[test]
    fn returns_none_if_chars_vary_by_more_than_one() {
        let actual = try_find_matching("abc", "ade");

        assert_eq!(None, actual);
    }

    #[test]
    fn finds_matching_ids_that_vary_by_one_char() {
        let test_case = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
        let actual = compare_ids(test_case);

        assert_eq!("fgij".to_string(), actual);
    }
}
