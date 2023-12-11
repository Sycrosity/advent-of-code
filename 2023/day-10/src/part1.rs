use std::collections::HashMap;

use glam::IVec2;

use tracing::info;

// use nom::branch::

use crate::custom_error::AocError;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let grid = to_grid(input);

    let starting_pos = grid
        .iter()
        .find_map(|(key, value)| (value == &PipeType::StartingPosition).then_some(key))
        .expect("there to be a starting position");

    info!(starting_pos = ?starting_pos);

    let mut valid_directions = Direction::iterator()
        .filter_map(|direction| {
            let position = *starting_pos + direction.to_vec();
            grid.get(&position)
                .is_some_and(|pipe| {
                    info!(pipe = ?pipe, direction = ?direction, pos = ?position);

                    direction.is_pipe_connected(pipe)
                })
                .then_some((position, *direction))
        })
        .map(|tuple| {
            std::iter::successors(Some(tuple), |(position, direction)| {
                let next_direction = grid
                    .get(position)
                    .expect("that this exists")
                    .to_direction(direction);

                info!(direction = ?direction, pos = ?position);

                Some((*position + next_direction.to_vec(), next_direction))
            })
        });

    let path_1 = valid_directions.next().expect("at least 2 paths");
    let path_2 = valid_directions.next().expect("at least 2 paths");

    Ok(path_1
        .zip(path_2)
        .position(|(a, b)| a.0 == b.0)
        .expect("an overlapping coordinate") as u32
        + 1)

    // Ok(0)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PipeType {
    StartingPosition,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}

impl PipeType {
    #[tracing::instrument]
    fn to_direction(&self, from_direction: &Direction) -> Direction {
        use Direction::*;
        use PipeType::*;

        info!("");

        let to = match (&self, from_direction) {
            (Vertical, South) => South,
            (Vertical, North) => North,
            (Horizontal, East) => East,
            (Horizontal, West) => West,
            (NorthEast, South) => East,
            (NorthEast, West) => North,
            (NorthWest, South) => West,
            (NorthWest, East) => North,
            (SouthWest, North) => West,
            (SouthWest, East) => South,
            (SouthEast, West) => South,
            (SouthEast, North) => East,
            _ => unreachable!(
                "Landing on {:?} from {from_direction:?} shouldn't be possible!",
                &self
            ),
        };

        info!(to = ?to);

        to
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn to_vec(&self) -> IVec2 {
        match &self {
            Direction::North => Point::NEG_Y,
            Direction::East => Point::X,
            Direction::South => Point::Y,
            Direction::West => Point::NEG_X,
        }
    }

    fn is_pipe_connected(&self, pipe: &PipeType) -> bool {
        match &self {
            Direction::North => 
                matches!(pipe, PipeType::Vertical | PipeType::SouthEast | PipeType::SouthWest),
            
            Direction::East => 
                matches!(pipe, PipeType::Horizontal | PipeType::SouthWest | PipeType::NorthWest),
            
            Direction::South => 
                matches!(pipe, PipeType::Vertical | PipeType::NorthEast | PipeType::NorthWest),
            
            Direction::West => matches!(pipe, PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast)
        }
    }

    pub fn iterator() -> std::slice::Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [North, East, South, West];
        DIRECTIONS.iter()
    }
}

type Point = IVec2;

type Grid = HashMap<Point, PipeType>;

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::StartingPosition,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            _ => Self::Ground,
        }
    }
}

fn to_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(column, line)| {
            line.chars().enumerate().map(move |(row, ch)| {
                let pos = Point::new(row as i32, column as i32);
                let pipe_type = PipeType::from(ch);

                (pos, pipe_type)
            })
        })
        .collect::<Grid>()
}

fn display(c: char) -> &'static str {
    match c {
        'S' => "S",
        '|' => "│",
        '-' => "─",
        'L' => "└",
        'J' => "┘",
        '7' => "┐",
        'F' => "┌",
        _ => "·",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() -> miette::Result<()> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
