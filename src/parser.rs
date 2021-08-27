pub mod annotations;
pub mod class_decl;
pub mod const_decl;
pub mod global_decl;
pub mod interface_decl;
pub mod module_decl;
pub mod signature;
pub mod r#type;
pub mod type_decl;
pub mod type_with_path;
pub mod whitespace;

pub type ParseResult<'input, O> = Result<(O, &'input str), &'input str>;

pub trait Parse<'input> {
    type Output;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output>;
}

macro_rules! parse_kw {
    ($struct_name:ident, $match:expr, $start_of_next:expr) => {
        pub struct $struct_name<'input> {
            _blah: std::marker::PhantomData<&'input str>,
        }

        impl<'input> crate::parser::Parse<'input> for $struct_name<'input> {
            type Output = ();

            fn parse(input: &'input str) -> crate::parser::ParseResult<'input, Self::Output> {
                if input.starts_with($match) {
                    return Ok(((), &input[$start_of_next..]));
                }

                Err(input)
            }
        }
    };
}

parse_kw!(KwEq, "= ", 2);
parse_kw!(KwType, "type ", 5);
