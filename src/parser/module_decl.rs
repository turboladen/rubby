use super::{Parse, ParseResult};

pub struct ModuleDecl<'input> {
    meow: &'input str,
}

impl<'input> Parse<'input> for ModuleDecl<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
