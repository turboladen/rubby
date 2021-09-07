use self::ast::declarations::Declaration;
use pest::Parser;
use std::ops::RangeTo;

// #[macro_use]
// extern crate pest;

pub mod ast;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "parser/rbs.pest"]
pub struct RbsParser;

pub fn parse<'input>(input: &'input str) -> Result<Vec<Declaration<'input>>, String> {
    todo!();
}

pub enum Location<'input> {
    WithChildren {
        buffer: String,
        start_pos: usize,
        end_pos: usize,
        optional_children: Vec<()>,
        required_children: Vec<()>,
        start_loc: usize,
        source: &'input str,
    },
}

pub struct TypeName<'input> {
    namespace: Namespace,
    name: &'input str,
    kind: TypeKind,
}

pub enum TypeKind {
    Class,
}

pub struct Namespace {
    path: Vec<()>,
    absolute: bool,
}

pub struct Buffer<'input> {
    name: Option<String>,
    content: &'input str,
    lines: Vec<&'input str>,
    ranges: Vec<RangeTo<usize>>,
}
