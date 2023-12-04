use crate::prelude::*;

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    Ok(
        input
            .lines() //split by newlines into an iterator of lines
            .map(|line| {
                let mut numbers = line
                    .chars() //split into an iter of char's
                    .filter_map(|char| char.to_digit(10)); //for each char, attempt to convert it to a base 10 integer (u32), else discard it.

                let first = numbers.next().expect("there to be at least 1 number");

                match numbers.last() {
                    Some(last) => first * 10 + last, //the first number * 10 + the last is equivelant to concatting the first and last numbers.
                    None => first * 10 + first, //if there is no `.last()` item (as `.first()` consumes the first item), then there must be only one number
                }
            })
            .sum(), //sum the iterator of u32's into one value.
    )
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
