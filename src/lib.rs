mod rbs;

use crate::rbs::parser;
use rutie::{methods, module, AnyObject, Array, Object, RString, VM};

#[no_mangle]
pub extern "C" fn init_rubby() {
    VM::init();
    VM::init_loadpath();
    rutie::VM::require("rbs");
    rutie::VM::require("rbs/parser");
}

// module!(Rubby);

// #[rustfmt::skip]
// methods!(
//     Rubby,
//     _itself,

//     fn check(signatures: Array) -> CheckResults {

//         tokio::runtime::Builder::new_multi_thread()
//             .enable_all()
//             .build()
//             .unwrap()
//             .block_on(async {
//                 _check(signatures).await
//             })
//     }
// );

// async fn _check(signatures: Array) -> CheckResults {
//     let mut sigs = Vec::new();

//     for signature in signatures {
//         sigs.push(parser::parse_signature(signature.try_convert_to::<RString>().unwrap().to_str()).await);
//     }

//     CheckResults { sigs }
// }

// pub async fn __check(signatures: &[&str]) -> CheckResults {
//     let mut sigs = Vec::new();

//     for signature in signatures {
//         sigs.push(parser::parse_signature(signature).await)
//     }

//     CheckResults { sigs }
// }

pub fn __sync_check(signatures: &[&str]) -> CheckResults {
    println!("pants");
    let mut sigs = Vec::new();

    for signature in signatures {
        sigs.push(parser::parse_signature_sync(signature))
    }

    CheckResults { sigs }
}

pub struct CheckResults {
    sigs: Vec<Vec<AnyObject>>,
}
