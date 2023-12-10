use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument(skip_all)]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let mut line = line
                .split_whitespace()
                .map(|numbers| numbers.parse::<i32>().expect("these to all be parseable"))
                .collect_vec();

            let mut total = 0;

            while !line.iter().all(|num| num == &0) {
                total += *line.last().unwrap();

                line = line
                    .iter()
                    .tuple_windows::<(&i32, &i32)>()
                    .map(|(a, b)| b - a)
                    .collect_vec();
            }

            total
        })
        .sum::<i32>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, process(input)?);
        Ok(())
    }
}
