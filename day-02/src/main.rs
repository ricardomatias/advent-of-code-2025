use std::io::{self, BufRead};
use std::{fs::File, io::BufReader};

fn sub_string(string: &str, len: usize) -> Vec<String> {
    let mut chars = string.chars();

    (0..)
        .map(|_| chars.by_ref().take(len).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>()
}

// TODO: Get [N] digits and then repeat them for the length of the id. If it matches the id, it's invalid.
fn is_invalid(id: &str) -> bool {
    let len = id.len();
    let end = len / 2;

    for n in 1..=end {
        let mut repeats: i8 = 1;
        let mut invalid = true;
        let digits = sub_string(id, n);

        'inner: for (i, curr) in digits.iter().by_ref().enumerate() {
            if i == 0 {
                continue;
            }

            if let Some(prev) = digits.get(i - 1) {
                if prev != curr {
                    invalid = false;
                    break 'inner;
                } else {
                    repeats += 1;
                }
            }
        }

        if invalid && repeats >= 2 {
            return true;
        }
    }

    false
}

fn verify_range(range: &str) -> Vec<i64> {
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

    invalid_ids
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
            // otherwise it fails on the last line
            .trim_end()
            .to_owned();

        let invalid = verify_range(&id_range);

        invalid_ids.extend(invalid);
    }

    println!(
        "The sum of all invalid ids is {}",
        invalid_ids.iter().sum::<i64>()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{is_invalid, verify_range};

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid("1212121212"));
    }

    #[test]
    fn test_ranges_day_one() {
        assert_eq!(verify_range("11-22"), vec![11, 22]);
        assert_eq!(verify_range("95-115"), vec![99, 111]);
        assert_eq!(verify_range("1188511880-1188511890"), vec![1188511885]);
        assert_eq!(verify_range("222220-222224"), vec![222222]);
        assert_eq!(verify_range("1698522-1698528"), vec![]);
        assert_eq!(verify_range("53-77"), vec![55, 66, 77]);
    }
    #[test]
    fn test_ranges_day_two() {
        let fixtures = vec![
            ("11-22", vec![11, 22]),
            ("95-115", vec![99, 111]),
            ("998-1012", vec![999, 1010]),
            ("1188511880-1188511890", vec![1188511885]),
            ("222220-222224", vec![222222]),
            ("1698522-1698528", vec![]),
            ("446443-446449", vec![446446]),
            ("565653-565659", vec![565656]),
            ("2121212118-2121212124", vec![2121212121]),
        ];

        let mut invalid_ids: Vec<i64> = Vec::new();

        for (range, result) in fixtures {
            let ids = verify_range(range);
            assert_eq!(ids, result);

            invalid_ids.extend(ids);
        }
    }
}
