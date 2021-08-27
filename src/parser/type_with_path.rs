use super::{Parse, ParseResult};

pub struct TypeWithPath<'input> {
    value: &'input str,
}

impl<'input> TypeWithPath<'input> {
    pub fn value(&self) -> &'input str {
        self.value
    }
}

impl<'input> Parse<'input> for TypeWithPath<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        let mut chars = input.chars();
        let mut offset = 0;
        let mut found_first_colon = false;

        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                break;
            } else if c == ':' {
            } else {
                offset += 1;
            }
        }
        dbg!(&input[..offset]);

        Ok((
            Self {
                value: &input[..offset],
            },
            &input[offset..],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse {
        ($test_name:ident, $input:expr, $parsed:expr, $tail:expr) => {
            #[test]
            fn $test_name() {
                let (ident, tail) = TypeWithPath::parse($input).unwrap();
                assert_eq!(ident.value(), $parsed);
                assert_eq!(tail, $tail);
            }
        };
    }

    test_parse!(test_class_name, "ClassName", "ClassName", "");
}
