use std::marker::PhantomData;

use super::{Parse, ParseResult};

pub struct Whitespace<'input> {
    _blah: PhantomData<&'input str>,
}

impl<'input> Parse<'input> for Whitespace<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        todo!()
    }
}
