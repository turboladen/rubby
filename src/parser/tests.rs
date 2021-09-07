mod types;

use super::*;
use pest::{consumes_to, parses_to};

#[test]
fn path_element_test() {
    let ruby = "Foo::Bar";

    parses_to! {
        parser: RbsParser, input: ruby, rule: Rule::path_element,
        tokens: [path_element(0, 3)]
    };
}

#[test]
fn namespace_test() {
    let ruby = "Foo::Bar";

    parses_to! {
        parser: RbsParser, input: ruby, rule: Rule::namespace,
        tokens: [namespace(0, 5, [
            path_element(0, 3)
        ])]
    };

    let ruby = "Foo::Bar::Baz";

    parses_to! {
        parser: RbsParser, input: ruby, rule: Rule::namespace,
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
        parser: RbsParser, input: ruby, rule: Rule::type_name,
        tokens: [type_name(0, 3, [
            path_element(0, 3)
        ])]
    };

    let ruby = "Foo::Bar";

    parses_to! {
        parser: RbsParser, input: ruby, rule: Rule::type_name,
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
        parser: RbsParser, input: ruby, rule: Rule::type_name,
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
        parser: RbsParser, input: ruby, rule: Rule::module,
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

#[test]
fn method_name_test() {
    parses_to! {
        parser: RbsParser, input: "foo", rule: Rule::method_name,
        tokens: [method_name(0, 3)]
    };

    parses_to! {
        parser: RbsParser, input: "foo_bar_baz", rule: Rule::method_name,
        tokens: [method_name(0, 11)]
    };
}

#[test]
fn parameter_test() {
    parses_to! {
        parser: RbsParser, input: "Object foo", rule: Rule::parameter,
        tokens: [parameter(0, 9, [
            t_type(0, 5),
            var_name(6, 9)
        ])]
    };
}

#[test]
fn var_name_test() {
    assert!(RbsParser::parse(Rule::var_name, "123foo").is_err());
    assert!(RbsParser::parse(Rule::var_name, "Foo").is_err());

    parses_to! {
        parser: RbsParser, input: "foo", rule: Rule::var_name,
        tokens: [var_name(0, 3)]
    };

    // The question mark doesn't get parsed, but erroring there is handled at a higher rule.
    parses_to! {
        parser: RbsParser, input: "foo?", rule: Rule::var_name,
        tokens: [var_name(0, 3)]
    };

    parses_to! {
        parser: RbsParser, input: "foo_bar_baz", rule: Rule::var_name,
        tokens: [var_name(0, 11)]
    };

    parses_to! {
        parser: RbsParser, input: "fooBar", rule: Rule::var_name,
        tokens: [var_name(0, 6)]
    };
}
