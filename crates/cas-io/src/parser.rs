use std::convert::Infallible;
use std::fmt::Display;

pub trait Parser {
    type Output: Display;
    type Error: Display;

    fn parse(&self, input: &str) -> Result<Self::Output, Self::Error>;

    fn commit(&mut self, input: &str) -> Result<Self::Output, Self::Error> {
        self.parse(input)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Echo;

impl Parser for Echo {
    type Output = String;
    type Error = Infallible;

    fn parse(&self, input: &str) -> Result<Self::Output, Self::Error> {
        Ok(input.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_returns_input_verbatim() {
        assert_eq!(Echo.parse("x^2 + 1").unwrap(), "x^2 + 1");
    }
}
