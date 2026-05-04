/// A compass direction
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Parse a direction from a single character
/// e.g. 'N' for North, 'S' for South, etc.
/// anything else is None
fn parse_direction(c: char) -> Option<Direction> {
    todo!()
}

/// Return the opposite of given direction
/// e.g. North -> South, East -> West, etc.
fn opposite(d: Direction) -> Direction {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_known() {
        assert_eq!(parse_direction('N'), Some(Direction::North));
        assert_eq!(parse_direction('S'), Some(Direction::South));
        assert_eq!(parse_direction('E'), Some(Direction::East));
        assert_eq!(parse_direction('W'), Some(Direction::West));
    }

    #[test]
    fn test_parse_unknown() {
        assert_eq!(parse_direction('X'), None);
        assert_eq!(parse_direction('n'), None);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(opposite(Direction::North), Direction::South);
        assert_eq!(opposite(Direction::South), Direction::North);
        assert_eq!(opposite(Direction::East), Direction::West);
        assert_eq!(opposite(Direction::West), Direction::East);
    }
}
