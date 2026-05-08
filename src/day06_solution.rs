// Enums in Rust model a closed set of variants — far more powerful than Python's enum,
// because each variant can carry its own data and `match` enforces exhaustiveness.
// `derive` auto-implements traits: Debug for printing, PartialEq for equality checks.
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

// `Option<T>` is the idiomatic "maybe" in Rust — there is no null. Either `Some(value)`
// or `None`. The compiler forces you to handle both cases.
fn parse_direction(c: char) -> Option<Direction> {
    match c {
        'N' => Some(Direction::North),
        'S' => Some(Direction::South),
        'E' => Some(Direction::East),
        'W' => Some(Direction::West),
        _ => None,
    }
}

// `match` is exhaustive — try removing one arm and the compiler will error.
// This is one of Rust's biggest safety wins over `if/elif` chains.
fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
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
