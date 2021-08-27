use super::{Parse, ParseResult};

pub struct InterfaceDecl<'input> {
    meow: &'input str,
}

impl<'input> Parse<'input> for InterfaceDecl<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
