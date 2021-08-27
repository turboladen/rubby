use super::{Parse, ParseResult};

pub struct ConstDecl<'input> {
    meow: &'input str,
}

impl<'input> Parse<'input> for ConstDecl<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
