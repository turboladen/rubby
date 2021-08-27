use super::{Parse, ParseResult};

pub struct Annotations<'input> {
    meow: &'input str,
}

impl<'input> Parse<'input> for Annotations<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
