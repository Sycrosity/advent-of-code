use crate::prelude::*;

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    Ok(input
        .lines()
        .map(process_line) //for each line, plug it into `proccess_line`
        .sum::<usize>())
}

/// Returns the
#[tracing::instrument]
fn process_line(line: &str) -> usize {
    /// all possible numbers in their word forms.
    const NUMBERWORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let line_len = line.len();
    let nums: Vec<usize> = (0..line_len) //iterate for the length of `line`
        .filter_map(|index| {
            let ch = line.as_bytes()[index]; //the character at the index `index` of this line.
            if ch.is_ascii_digit() {
                return Some(
                    (ch as usize) - 48, // -48 for the ascii offset
                );
            }

            NUMBERWORDS.iter().enumerate().find_map(|(i, numberword)| {
                //check if a line slice beginning from the index starts with one of the numbers in word form
                if (line_len - index) >= 3 && line[index..].starts_with(numberword) {
                    Some(
                        i + 1, // +1 as NUMBERWORDS starts with index 0, but the first term is "one"
                    )
                } else {
                    None // as this is a `filter_map()`, returning `None` will remove this value from the iterator.
                }
            })
        })
        .collect();

    let first = nums.first().expect("there to be at least 1 number");
    let last = nums.last().expect("there to be at least 1 number");

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
    fn line_test(#[case] line: &str, #[case] expected: usize) {
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
