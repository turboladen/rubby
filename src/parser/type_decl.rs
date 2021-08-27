use super::{
    annotations::Annotations, type_with_path::TypeWithPath, whitespace::Whitespace, KwEq, KwType,
    Parse, ParseResult,
};

pub struct TypeDecl<'input> {
    annotations: Annotations<'input>,
    type_alias_name: &'input str,
    type_name: &'input str,
}

impl<'input> Parse<'input> for TypeDecl<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        let (annotations, annotation_tail) = Annotations::parse(input)?;
        let (_, whitespace_tail) = Whitespace::parse(annotation_tail)?;
        let (_, type_tail) = KwType::parse(whitespace_tail)?;
        let (type_alias_name, tan_tail) = TypeWithPath::parse(type_tail)?;
        let (_, eq_tail) = KwEq::parse(tan_tail)?;
        let (type_name, type_tail) = TypeWithPath::parse(eq_tail)?;

        Ok((
            Self {
                annotations,
                type_alias_name: type_alias_name.value(),
                type_name: type_name.value(),
            },
            type_tail,
        ))
    }
}
