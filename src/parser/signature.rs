use super::{
    class_decl::ClassDecl, const_decl::ConstDecl, global_decl::GlobalDecl,
    interface_decl::InterfaceDecl, module_decl::ModuleDecl, type_decl::TypeDecl, Parse,
    ParseResult,
};

pub enum Signature<'input> {
    Type(TypeDecl<'input>),
    Const(ConstDecl<'input>),
    Global(GlobalDecl<'input>),
    Interface(InterfaceDecl<'input>),
    Module(ModuleDecl<'input>),
    Class(ClassDecl<'input>),
}

impl<'input> Parse<'input> for Signature<'input> {
    type Output = Self;

    fn parse(input: &'input str) -> ParseResult<'input, Self::Output> {
        if let Ok((type_decl, rest)) = TypeDecl::parse(input) {
            return Ok((Self::Type(type_decl), rest));
        }

        if let Ok((const_decl, rest)) = ConstDecl::parse(input) {
            return Ok((Self::Const(const_decl), rest));
        }

        if let Ok((global_decl, rest)) = GlobalDecl::parse(input) {
            return Ok((Self::Global(global_decl), rest));
        }

        if let Ok((interface_decl, rest)) = InterfaceDecl::parse(input) {
            return Ok((Self::Interface(interface_decl), rest));
        }

        if let Ok((module_decl, rest)) = ModuleDecl::parse(input) {
            return Ok((Self::Module(module_decl), rest));
        }

        if let Ok((class_decl, rest)) = ClassDecl::parse(input) {
            return Ok((Self::Class(class_decl), rest));
        }

        Err(input)
    }
}
