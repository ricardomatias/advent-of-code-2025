use std::io::{self, BufRead};
use std::{fs::File, io::BufReader};

const ZERO: u8 = b'0';

fn main() -> io::Result<()> {
    let total = extract_output_voltage("input.txt");

    println!("The total output voltage is {total}");
    Ok(())
}

fn extract_output_voltage(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_output: u32 = 0;

    for bank in reader.split(b'\n') {
        let bank = bank.unwrap();
        let mut a: u8 = 0;

        for &bat in bank.iter() {
            if bat > a {
                a = bat;
            }
        }

        let mut a_large = true;
        let a_index = (bank.iter().position(|b| b == &a)).unwrap();
        let range = if a_index == bank.len() - 1 {
            a_large = false;
            0..a_index
        } else {
            (a_index + 1)..bank.len()
        };

        let mut b: u8 = 0;

        for &bat in bank[range.clone()].iter() {
            if bat > b {
                b = bat;
            }
        }

        let joltage = if a_large {
            (a - ZERO) as u32 * 10 + (b - ZERO) as u32
        } else {
            (b - ZERO) as u32 * 10 + (a - ZERO) as u32
        };

        total_output += joltage;
    }

    total_output
}

#[cfg(test)]
mod tests {
    use crate::extract_output_voltage;

    #[test]
    fn test_fixture() {
        assert_eq!(extract_output_voltage("test.txt"), 412);
    }
}
