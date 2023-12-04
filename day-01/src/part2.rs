use crate::prelude::*;

const NUMBERWORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input.lines().map(process_line).sum::<u32>())
}

fn process_line(line: &str) -> u32 {
    let nums: Vec<u32> = (0..line.len())
        .filter_map(|index| {
            let ch = &line.as_bytes()[index];
            if ch.is_ascii_digit() {
                return Some(*ch as u32 - 48);
            }

            NUMBERWORDS.iter().enumerate().find_map(|(i, numberword)| {
                if line[index..].starts_with(numberword) {
                    Some(i as u32)
                } else {
                    None
                }
            })
        })
        .collect();

    let first = nums.first().expect("there to be at least 1 number");
    let last = nums.last().expect("there to be at least 1 number");

    // String::from_utf8_lossy(&[(first + 48) as u8, (last + 48) as u8])
    // .parse::<u32>()
    // .expect("this concat to be a valid number")
    first * 10 + last
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
