use std::convert::Infallible;

use cas_io::Parser;

#[derive(Debug, Default, Clone, Copy)]
pub struct Evaluator;

impl Parser for Evaluator {
    type Output = String;
    type Error = Infallible;

    fn parse(&self, input: &str) -> Result<Self::Output, Self::Error> {
        Ok(input.to_owned())
    }
}
