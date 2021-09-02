use self::ast::declarations::Declaration;
use pest::Parser;
use std::ops::RangeTo;

// #[macro_use]
// extern crate pest;

pub mod ast;

#[derive(Parser)]
#[grammar = "parser/rbs.pest"]
pub struct RbsParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::{consumes_to, parses_to};

    #[test]
    fn path_element_test() {
        let ruby = "Foo::Bar";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::path_element,
            tokens: [path_element(0, 3)]
        };
    }

    #[test]
    fn namespace_test() {
        let ruby = "Foo::Bar";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::namespace,
            tokens: [namespace(0, 5, [
                path_element(0, 3)
            ])]
        };

        let ruby = "Foo::Bar::Baz";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::namespace,
            tokens: [namespace(0, 10, [
                path_element(0, 3),
                path_element(5, 8)
            ])]
        };
    }

    #[test]
    fn type_name_test() {
        let ruby = "Bar";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::type_name,
            tokens: [type_name(0, 3, [
                path_element(0, 3)
            ])]
        };

        let ruby = "Foo::Bar";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::type_name,
            tokens: [type_name(0, 8, [
                namespace(0, 5, [
                    path_element(0, 3)
                ]),
                path_element(5, 8)
            ]
            )]
        };

        let ruby = "Foo::Bar::Baz";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::type_name,
            tokens: [type_name(0, 13, [
                namespace(0, 10, [
                    path_element(0, 3),
                    path_element(5, 8)
                ]),
                path_element(10, 13)
            ])]
        };
    }

    #[test]
    fn module_test() {
        let ruby = "module Bar; end";
        assert!(RbsParser::parse(Rule::module, ruby).is_err());

        let ruby = "module Bar\nend";

        parses_to! {
            parser: RbsParser,
            input:  ruby,
            rule:   Rule::module,
            tokens: [module(0, 14, [
                type_name(7, 10, [
                    path_element(7, 10)
                ])
            ])]
        };

        // let ruby = r#"module Bar
        //         def foo: -> void
        //     end"#;

        // parses_to! {
        //     parser: RbsParser,
        //     input:  ruby,
        //     rule:   Rule::module,
        //     tokens: [module(0, 15, [
        //         type_name(7, 10, [
        //             path_element(8, 11)
        //         ])
        //     ])]
        // };
    }
}
