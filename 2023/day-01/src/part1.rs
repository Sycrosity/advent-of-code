use crate::prelude::*;

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|char| char.to_digit(10));

            let first = numbers.next().expect("there to be at least 1 number");

            match numbers.last() {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
