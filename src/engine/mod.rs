mod tokenizer;
mod evaluator;
mod tree_builder;

struct Juxt {
    name: String,
    template: String,
}

pub fn execute(juxt: &str) -> String {
    let js_code = compile(juxt, "execute");
    let result = evaluator::eval(&js_code);
    return result;
}

fn compile(juxt: &str, fn_name: &str) -> String {
    let tokens = tokenizer::tokenize(juxt);
    let nodes = tree_builder::build_tree(&tokens);
    let js_code = evaluator::generate_js(fn_name, "context", &nodes, &compile);
    return js_code;
}
