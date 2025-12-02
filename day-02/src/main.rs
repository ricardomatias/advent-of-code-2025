use std::io::{self, BufRead};
use std::{fs::File, io::BufReader};

fn is_invalid(id: &str) -> bool {
    let len = id.len();
    if len.is_multiple_of(2) {
        let (pre, post) = id.split_at(len / 2);

        pre == post
    } else {
        false
    }
}

fn verify_range(range: &str) -> Option<Vec<i64>> {
    let mut invalid_ids: Vec<i64> = Vec::new();
    let (start, end) = range.split_once("-").unwrap();
    let i_start = start.parse::<i64>().unwrap();
    let i_end = end
        .parse::<i64>()
        .map_err(|e| eprintln!("Failed to parse {end} - {}", e))
        .unwrap();

    for id in i_start..=i_end {
        if is_invalid(&id.to_string()) {
            invalid_ids.push(id);
        }
    }

    Some(invalid_ids)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut invalid_ids: Vec<i64> = Vec::new();

    for id_range in reader.split(b',') {
        if id_range.is_err() {
            panic!("Failed to parse id range");
        }

        let id_range = str::from_utf8(&id_range.unwrap())
            .unwrap()
            .trim_end()
            .to_owned();
        print!("Checking {}. Invalid = ", id_range);
        if let Some(invalid) = verify_range(&id_range) {
            for id in invalid.iter() {
                print!("{id}, ");
            }
            println!();
            invalid_ids.extend(invalid);
        }
    }

    println!(
        "The sum of all invalid ids is {}",
        invalid_ids.iter().sum::<i64>()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::verify_range;

    #[test]
    fn test_ranges() {
        assert_eq!(verify_range("11-22"), Some(vec![11, 22]));
        assert_eq!(verify_range("95-115"), Some(vec![99]));
        assert_eq!(
            verify_range("1188511880-1188511890"),
            Some(vec![1188511885])
        );
        assert_eq!(verify_range("222220-222224"), Some(vec![222222]));
        assert_eq!(verify_range("1698522-1698528"), Some(vec![]));
        assert_eq!(verify_range("53-77"), Some(vec![55, 66, 77]));
    }
}
