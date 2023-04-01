mod tokenizer;
mod tree_builder;
mod tree_parser;

pub fn execute(val: &str) -> String {
    let tokens = tokenizer::tokenize(val);
    let tree = tree_builder::build_tree(&tokens);
    return String::from("Hello, world!");
}
