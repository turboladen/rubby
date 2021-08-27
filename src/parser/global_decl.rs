use super::{Parse, ParseResult};

pub struct GlobalDecl<'input> {
    meow: &'input str,
}

impl<'input> Parse<'input> for GlobalDecl<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
