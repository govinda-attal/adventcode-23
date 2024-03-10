use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|ch| ch.to_digit(10));
            let first = it.next().expect("should be a number");
            let last = match it.last() {
                Some(n) => n,
                None => first,
            };
            10 * first + last
        })
        .sum::<u32>();

    Ok(output.to_string())
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
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
