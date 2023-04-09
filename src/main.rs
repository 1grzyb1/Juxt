use juxt_core::compile_and_execute;
use juxt_core::engine::Juxt;

fn main() {
    let template = "Hello World";
    let result = compile_and_execute(
        Juxt {
            name: "main".to_string(),
            template: template.to_string(),
        },
        Vec::new(),
        String::new(),
    );
    println!("{}", result.unwrap());
}
