use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// ROTATIONS
/// L - left (lower numbers)
/// R - right (higher numbers)
/// XX - distance
/// Default - 50
/// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.
///

#[derive(Debug)]
struct Dial {
    position: i16,
    password: i16,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            position: 50,
            password: 0,
        }
    }
}

impl Dial {
    fn rotate(&mut self, dir: Direction, dist: i16) {
        match dir {
            Direction::Left => {
                self.position = ((self.position - dist) % 100 + 100) % 100;
            }
            Direction::Right => {
                self.position = (self.position + dist) % 100;
            }
        }

        if self.position == 0 {
            self.password += 1;
        }
    }
}

enum Direction {
    Left,
    Right,
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut dial = Dial::default();

    for line in lines {
        let line = line.unwrap();
        let (direction, distance) = line.split_at(1);
        let direction = if direction == "L" {
            Direction::Left
        } else {
            Direction::Right
        };
        let distance = distance.parse::<i16>().unwrap();

        dial.rotate(direction, distance);
    }

    println!("Password is: {}", dial.password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_dial_rotate() {
        let mut dial = Dial::default();

        dial.rotate(Direction::Left, 68);
        assert_eq!(dial.position, 82);

        dial.rotate(Direction::Left, 30);
        assert_eq!(dial.position, 52);
        assert_eq!(dial.password, 0);

        dial.rotate(Direction::Right, 48);
        assert_eq!(dial.position, 0);
        assert_eq!(dial.password, 1);

        dial.rotate(Direction::Left, 102);
        assert_eq!(dial.position, 98);

        dial.rotate(Direction::Right, 2);
        assert_eq!(dial.position, 0);
        assert_eq!(dial.password, 2);
    }
}
