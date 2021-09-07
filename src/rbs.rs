pub(crate) mod parser;

use rutie::{class, Module, Object, RString, VerifiedObject};

class!(Alias);

impl VerifiedObject for Alias {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class()
            == Module::from_existing("RBS")
                .get_nested_module("AST")
                .get_nested_module("Declarations")
                .get_nested_class("Alias")
    }

    fn error_message() -> &'static str {
        "Error converting to RBS::AST::Declarations::Alias"
    }
}

impl Alias {
    pub(crate) fn name(&self) -> String {
        let raw_type_name = unsafe { self.send("name", &[]) };
        let type_name = raw_type_name.try_convert_to::<TypeName>().unwrap();
        type_name.name()
    }
}

class!(TypeName);

impl VerifiedObject for TypeName {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Module::from_existing("RBS").get_nested_class("TypeName")
    }

    fn error_message() -> &'static str {
        "Error converting to RBS::TypeName"
    }
}

impl TypeName {
    pub(crate) fn namespace(&self) -> String {
        let namespace = unsafe { self.send("namespace", &[]) };
        let namespace = unsafe { namespace.send("to_s", &[]) };
        let ruby_namespace = namespace.try_convert_to::<RString>().unwrap();

        ruby_namespace.to_string()
    }

    pub(crate) fn name(&self) -> String {
        // let name = self.instance_variable_get("@name");
        let name = unsafe { self.send("to_s", &[]) };
        let ruby_name = name.try_convert_to::<RString>().unwrap();

        ruby_name.to_string()
    }
}
