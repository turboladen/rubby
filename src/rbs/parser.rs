use rutie::{class, AnyObject, Array, Class, Module, Object, RString, VerifiedObject};

class!(Parser);

impl VerifiedObject for Parser {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Module::from_existing("RBS").get_nested_class("Parser")
    }

    fn error_message() -> &'static str {
        "Error converting to RBS::Parser"
    }
}

impl Parser {
    pub(crate) fn class() -> Class {
        Module::from_existing("RBS").get_nested_class("Parser")
    }
}

pub(crate) async fn parse_signature(signature: &str) -> Vec<AnyObject> {
    Parser::class()
        .protect_public_send("parse_signature", &[RString::new_utf8(signature).to_any_object()])
        .unwrap()
        .try_convert_to::<Array>()
        .unwrap()
        .into_iter()
        .collect()
}

pub(crate) fn parse_signature_sync(signature: &str) -> Vec<AnyObject> {
    // Parser::class()
    //     .protect_public_send("parse_signature", &[RString::new_utf8(signature).to_any_object()])
    //     .unwrap()
    //     .try_convert_to::<Array>()
    //     .unwrap()
    //     .into_iter()
    //     .collect()
    println!("meow");
    let class = Parser::class();

    dbg!(&class);

    class
        .protect_public_send("parse_signature", &[RString::new_utf8(signature).to_any_object()])
        .unwrap()
        .try_convert_to::<Array>()
        .unwrap()
        .into_iter()
        .collect()
}
