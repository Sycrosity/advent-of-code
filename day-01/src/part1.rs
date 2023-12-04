use crate::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|char| char.to_digit(10));

            let first = numbers.next().expect("there to be at least 1 number");

            match numbers.last() {
                Some(last) => format!("{first}{last}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("this concat to be a valid number")
        })
        .fold(0, |acc, num| acc + num))
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
