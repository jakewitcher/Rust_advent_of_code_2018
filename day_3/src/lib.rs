#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(id: usize, x_offset: usize, y_offset: usize, width: usize, height: usize) -> Claim {
        Claim { id, x_offset, y_offset, width, height }
    }
}

fn parse_claim(line: &str) -> Option<Claim> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let id = parse_id(parts[0]);
    let offset = parse_offset(parts[2]);
    let area = parse_area(parts[3]);

    if let (Some(id), Some((x_offset, y_offset)), Some((width, height))) = (id, offset, area) {
        Some(Claim::new(id, x_offset, y_offset, width, height))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_id_from_string() {
        let expected = Some(123);
        let actual = parse_id("#123");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_offset_from_string() {
        let expected = Some((3, 2));
        let actual = parse_offset("3,2:");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_area_from_string() {
        let expected = Some((5, 4));
        let actual = parse_area("5x4");

        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_claim_from_string() {
        let expected = Some(Claim::new(123, 3, 2, 5, 4));
        let actual = parse_claim("#123 @ 3,2: 5x4");

        assert_eq!(expected, actual);
    }
}
