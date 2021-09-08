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
fn t_string_literal_test() {
    assert!(RbsParser::parse(Rule::t_symbol_literal, "123").is_err());

    // Double-quote strings
    {
        parses_to! {
            parser: RbsParser, input:  r#""""#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 2, [dq_string_content(1, 1)])]
        };

        parses_to! {
            parser: RbsParser, input:  r#""stuff""#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 7, [dq_string_content(1, 6)])]
        };

        parses_to! {
            parser: RbsParser, input:  r#""This is a sentence.""#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 21, [dq_string_content(1, 20)])]
        };

        parses_to! {
            parser: RbsParser, input:  r#""Ce m'est égale""#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 17, [dq_string_content(1, 16)])]
        };

        parses_to! {
            parser: RbsParser, input:  r#""私は亀が好きです""#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 26, [dq_string_content(1, 25)])]
        };

        // TODO: quote escapes
        // parses_to! {
        //     parser: RbsParser, input:  r#""And then I said, \"whoa!\"""#, rule: Rule::t_string_literal,
        //     tokens: [t_string_literal(0, 27, [dq_string_content(1, 26)])]
        // };
    }

    // Single-qoute strings
    {
        parses_to! {
            parser: RbsParser, input:  r#"''"#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 2, [sq_string_content(1, 1)])]
        };

        parses_to! {
            parser: RbsParser, input:  r#"'stuff'"#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 7, [sq_string_content(1, 6)])]
        };

        parses_to! {
            parser: RbsParser, input:  r#"'This is a sentence.'"#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 21, [sq_string_content(1, 20)])]
        };

        // TODO: quote escapes
        // parses_to! {
        //     parser: RbsParser, input:  r#"'Ce m\'est égale'"#, rule: Rule::t_string_literal,
        //     tokens: [t_string_literal(0, 17, [sq_string_content(1, 16)])]
        // };

        parses_to! {
            parser: RbsParser, input:  r#"'私は亀が好きです'"#, rule: Rule::t_string_literal,
            tokens: [t_string_literal(0, 26, [sq_string_content(1, 25)])]
        };
    }
}

#[test]
fn t_symbol_literal_test() {
    assert!(RbsParser::parse(Rule::t_symbol_literal, "foo").is_err());

    // Leading "@"
    {
        assert!(RbsParser::parse(Rule::t_symbol_literal, ":@1").is_err());

        parses_to! {
            parser: RbsParser, input:  ":@_", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3, [instance_variable(1, 3)])]
        };

        parses_to! {
            parser: RbsParser, input:  ":@a", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3, [instance_variable(1, 3)])]
        };

        parses_to! {
            parser: RbsParser, input:  ":@A", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3, [instance_variable(1, 3)])]
        };

        parses_to! {
            parser: RbsParser, input:  ":@a1", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3, [instance_variable(1, 3)])]
        };

        parses_to! {
            parser: RbsParser, input:  ":@a1", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3, [instance_variable(1, 3)])]
        };

        parses_to! {
            parser: RbsParser, input:  ":@_1", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 3, [instance_variable(1, 3)])]
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
            parser: RbsParser, input: ":thing", rule: Rule::t_symbol_literal,
            tokens: [t_symbol_literal(0, 6, [method_name(1, 6)])]
        };
    }
}
