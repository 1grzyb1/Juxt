mod tokenizer;
mod evaluator;
mod tree_builder;

pub struct Juxt {
    pub name: String,
    pub template: String,
}

pub fn execute(js_code: &str) -> String {
    return evaluator::eval(js_code);
}

pub fn compile(main: Juxt, dependencie: Vec<Juxt>) -> String {
    let tokens = tokenizer::tokenize(&main.template);
    let tree = tree_builder::build_tree(&tokens);
    let compiled_dependencies = compile_dependencies(&dependencie);
    let compiled_main = evaluator::generate_js("execute", "context", &tree, compiled_dependencies);
    return compiled_main;
}

fn compile_dependencies(juxts: &Vec<Juxt>) -> Vec<String> {
    let mut compiled = Vec::new();
    for juxt in juxts {
        let tokens = tokenizer::tokenize(&juxt.template);
        let tree = tree_builder::build_tree(&tokens);
        let compiled_juxt = evaluator::generate_js(&juxt.name, "context", &tree, Vec::new());
        compiled.push(compiled_juxt);
    }
    return compiled;
}
