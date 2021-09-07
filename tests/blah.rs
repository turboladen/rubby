// #[tokio::test]
// async fn function_name_test() {
//     let ruby = r#"def foo
//     puts 'hi'
//     end
//     "#;

//     let results = rubby::__check(&[ruby]).await;
// }
#[test]
fn function_name_test() {
    rubby::init_rubby();
    let ruby = r#"def foo
    puts 'hi'
    end
    "#;

    let results = rubby::__sync_check(&[ruby]);
}
