use super::{Parse, ParseResult};

pub struct ClassDecl<'input> {
    meow: &'input str,
}

impl<'input> Parse<'input> for ClassDecl<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
