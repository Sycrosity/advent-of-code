use tracing::info;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
