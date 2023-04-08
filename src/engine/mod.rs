use std::error::Error;

mod tokenizer;
mod evaluator;
mod tree_builder;

#[cfg(test)]
mod tests {
    mod evaluator_tests;
    mod tokenizer_tests;
    mod tree_builder_tests;
}

pub struct Juxt {
    pub name: String,
    pub template: String,
}

pub fn execute(js_code: &str) -> Result<String, Box<dyn Error>> {
    return evaluator::eval(js_code);
}

pub fn compile(main: Juxt, dependencie: Vec<Juxt>) -> Result<String, Box<dyn Error>> {
    let tokens = tokenizer::tokenize(&main.template);
    let tree = tree_builder::build_tree(&tokens);
    let compiled_dependencies = compile_dependencies(&dependencie)?;
    let compiled_main = evaluator::generate_js("execute", "context", &tree, compiled_dependencies);
    return compiled_main;
}

fn compile_dependencies(juxts: &Vec<Juxt>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut compiled = Vec::new();
    for juxt in juxts {
        let tokens = tokenizer::tokenize(&juxt.template);
        let tree = tree_builder::build_tree(&tokens);
        let compiled_juxt = evaluator::generate_js(&juxt.name, "context", &tree, Vec::new())?;
        compiled.push(compiled_juxt);
    }
    return Ok(compiled);
}
