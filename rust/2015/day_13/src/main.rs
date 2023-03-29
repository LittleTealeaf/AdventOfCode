use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

struct RuleInput {
    subject: String,
    net_happiness: i32,
    target: String,
}


#[derive(Debug)]
enum ParseError {
    NoneValue,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl TryFrom<String> for RuleInput {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut tokens = value.split(" ");
        let subject = String::from(tokens.next().ok_or(ParseError::NoneValue)?);
        tokens.next();
        tokens.next();
        let net_happiness = tokens.next().ok_or(ParseError::NoneValue)?.parse()?;

        for _ in 0..6 {
            tokens.next();
        }

        let target = String::from(tokens.next().ok_or(ParseError::NoneValue)?);

        Ok(RuleInput {
            subject,
            net_happiness,
            target,
        })
    }
}

fn main() {
    {
        let file = File::open("input.txt").unwrap();
        let input: Vec<RuleInput> = BufReader::new(file)
            .lines()
            .into_iter()
            .map(Result::unwrap)
            .map(RuleInput::try_from)
            .map(Result::unwrap)
            .collect();


    }
}

fn part_1(input: Vec<RuleInput>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}
}
