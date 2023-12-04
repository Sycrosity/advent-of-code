use std::u32;

use tracing::info;

use crate::prelude::*;

const NUMBERWORD_CHARS: [char; 14] = [
    'o', 'n', 'e', 't', 'w', 'h', 'r', 'f', 'u', 'i', 'v', 's', 'x', 'g',
];

const NUMBERWORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[inline]
fn is_numbermeric(ch: &char) -> bool {
    NUMBERWORD_CHARS.contains(ch)
}

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    info!("er");
    Ok(input
        .lines()
        .map(|line| {
            // let line = line.replace(&NUMBERWORD_CHARS[..], "");

            process_line(line)
        })
        .sum::<u32>())

    // Err(AocError::UnknownError)
}

fn process_line(line: &str) -> u32 {
    let nums: Vec<u32> = (0..line.len()).filter_map(|index| {

        let ch = &line.as_bytes()[index];
        if ch.is_ascii_digit() {

            return Some(*ch as u32 - 48);

        }

        NUMBERWORDS
            .iter()
            .enumerate()
            .find_map(|(i, numberword)| {
                if line[index..].starts_with(numberword) {
                    Some(i as u32)
                } else {
                    None
                }
            })
    }).collect();

    let first = nums.first().expect("there to be at least 1 number");
    let last = nums.last().expect("there to be at least 1 number");

    format!("{first}{last}")
    .parse::<u32>()
    .expect("this concat to be a valid number")

}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
