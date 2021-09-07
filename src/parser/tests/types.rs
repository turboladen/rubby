use super::*;
use pest::{consumes_to, parses_to};

#[test]
fn t_integer_literal_test() {
    assert!(RbsParser::parse(Rule::t_integer_literal, "foo").is_err());

    let ruby = "1_2_3";

    parses_to! {
        parser: RbsParser, input: ruby, rule: Rule::t_integer_literal,
        tokens: [
            t_integer_literal(0, 5),
        ]
    };

    let ruby = "1_2_3_";

    parses_to! {
        parser: RbsParser, input:  ruby, rule:   Rule::t_integer_literal,
        tokens: [
            t_integer_literal(0, 5),
        ]
    };
}

#[test]
fn t_symbol_literal_test() {
    assert!(RbsParser::parse(Rule::t_symbol_literal, "foo").is_err());

    // Leading "@"
    {
        assert!(RbsParser::parse(Rule::t_symbol_literal, ":@1").is_err());

        parses_to! {
            parser: RbsParser, input:  ":@_", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };

        parses_to! {
            parser: RbsParser, input:  ":@a", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };

        parses_to! {
            parser: RbsParser, input:  ":@A", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };

        parses_to! {
            parser: RbsParser, input:  ":@a1", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };

        parses_to! {
            parser: RbsParser, input:  ":@a1", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };

        parses_to! {
            parser: RbsParser, input:  ":@_1", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };
    }

    // Leading "$"
    {
        // assert!(RbsParser::parse(Rule::t_symbol_literal, ":$1a").is_err());

        // parses_to! {
        //     parser: RbsParser, input:  ":$_", rule: Rule::t_symbol_literal,
        //     tokens: [t_symbol_literal(0, 3)]
        // };
        // parses_to! {
        //     parser: RbsParser, input:  ":$1", rule: Rule::t_symbol_literal,
        //     tokens: [t_symbol_literal(0, 3)]
        // };
        // parses_to! {
        //     parser: RbsParser, input:  ":$@", rule: Rule::t_symbol_literal,
        //     tokens: [t_symbol_literal(0, 3)]
        // };
        // parses_to! {
        //     parser: RbsParser, input:  ":$!", rule: Rule::t_symbol_literal,
        //     tokens: [t_symbol_literal(0, 3)]
        // };
    }

    // Regular symbol
    {
        parses_to! {
            parser: RbsParser, input:  ":thing", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3)]
        };
    }
}
