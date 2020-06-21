#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
}

impl Claim {
    pub fn new(line: &str) -> Option<Claim> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let id = Claim::parse_id(parts[0]);
        let offset = Claim::parse_offset(parts[2]);
        let area = Claim::parse_area(parts[3]);

        if let (Some(id), Some((x_offset, y_offset)), Some((width, height))) = (id, offset, area) {
            Some(Claim { id, x_offset, y_offset, width, height })
        } else {
            None
        }
    }

    fn parse_id(part: &str) -> Option<usize> {
        part[1..].parse::<usize>().ok()
    }
    
    fn parse_offset(part: &str) -> Option<(usize, usize)> {
        if let Some(i) = part.find(',') {
            if let (Ok(x), Ok(y)) = (part[..i].parse::<usize>(), part[i + 1..part.len() - 1].parse::<usize>()) {
                return Some((x, y))
            }
        }
        
        None
    }
    
    fn parse_area(part: &str) -> Option<(usize, usize)> {
        if let Some(i) = part.find('x') {
            if let (Ok(x), Ok(y)) = (part[..i].parse::<usize>(), part[i + 1..].parse::<usize>()) {
                return Some((x, y))
            }
        }
        
        None
    }

    pub fn coords(&self) -> Vec<(usize, usize)> {
        let mut results = Vec::new();

        for x in self.x_offset..self.x_offset + self.width - 1 {
            for y in self.y_offset..self.y_offset + self.height - 1 {
                results.push((x, y));
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_id_from_string() {
        let expected = Some(123);
        let actual = Claim::parse_id("#123");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_offset_from_string() {
        let expected = Some((3, 2));
        let actual = Claim::parse_offset("3,2:");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_area_from_string() {
        let expected = Some((5, 4));
        let actual = Claim::parse_area("5x4");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_claim_from_string() {
        let expected = Some(Claim{ id: 123, x_offset: 3, y_offset: 2, width: 5, height: 4 });
        let actual = Claim::new("#123 @ 3,2: 5x4");

        assert_eq!(expected, actual);
    }

    #[test]
    fn returns_list_of_coords_for_claim() {
        let expected = Claim::new("#123 @ 3,2: 5x4").unwrap().coords();
        let actual = vec![
            (3, 2), (3, 3), (3, 4), 
            (4, 2), (4, 3), (4, 4), 
            (5, 2), (5, 3), (5, 4),
            (6, 2), (6, 3), (6, 4)
        ];

        assert_eq!(expected, actual);
    }
}
