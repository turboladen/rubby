use super::{Parse, ParseResult};

#[derive(Debug)]
pub struct Type<'input> {
    value: &'input str,
}

impl<'input> Type<'input> {
    pub fn value(&self) -> &'input str {
        self.value
    }
}

impl<'input> Parse<'input> for Type<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        let mut chars = input.chars();
        let mut offset = 0;

        if let Some(c) = chars.next() {
            if !c.is_uppercase() {
                return Err(input);
            }
            offset += 1;
        }

        while let Some(c) = chars.next() {
            if c.is_whitespace() || c == ':' {
                break;
            }

            offset += 1;

            if !c.is_alphanumeric() {
                return Err(&input[offset..]);
            }
        }

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
                let (ident, tail) = Type::parse($input).unwrap();
                assert_eq!(ident.value(), $parsed, "Consumed incorrectly");
                assert_eq!(tail, $tail, "Tail is incorrect");
            }
        };
    }

    macro_rules! test_parse_fail {
        ($test_name:ident, $input:expr, $unused:expr) => {
            #[test]
            fn $test_name() {
                let unused = Type::parse($input).unwrap_err();
                assert_eq!(unused, $unused, "unused is incorrect");
            }
        };
    }
    test_parse!(test_class_name, "ClassName", "ClassName", "");
    test_parse!(test_class_name_whitespace, "ClassName ", "ClassName", " ");
    test_parse!(
        test_class_name_whitespace_class_name,
        "Class Name ",
        "Class",
        " Name "
    );

    test_parse_fail!(test_all_lower_case, "class_name", "class_name");
}
