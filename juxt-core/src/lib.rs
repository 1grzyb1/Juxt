use crate::engine::{compile, execute, Juxt};

pub mod engine;

pub fn compile_and_execute(
    main: Juxt,
    dependencies: Vec<Juxt>,
    context: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let compiled = compile(main, dependencies)?;
    return execute(&compiled, context);
}

pub fn execute_js(js_code: &str, context: String) -> Result<String, Box<dyn std::error::Error>> {
    return execute(js_code, context);
}

pub fn compile_flux(
    main: Juxt,
    dependencies: Vec<Juxt>,
) -> Result<String, Box<dyn std::error::Error>> {
    return compile(main, dependencies);
}
