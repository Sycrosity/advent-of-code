#![allow(unused)]

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32, AocError> {
    Ok(input
    .lines()
    .map(|line| {
        let mut line = line
            .split_whitespace()
            .map(|numbers| numbers.parse::<i32>().expect("these to all be parseable"))
            .collect_vec();

        let mut first_nums = vec![];

        while !line.iter().all(|num| num == &0) {

            first_nums.push(*line.first().unwrap());

            line = line
                .iter()
                .tuple_windows::<(&i32, &i32)>()
                .map(|(a, b)| b - a)
                .collect_vec();
        }

        first_nums.iter().rev().fold(0, |acc,num| {

            num - acc

        })
    })
    .sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
