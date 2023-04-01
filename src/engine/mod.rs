mod tokenizer;
mod evaluator;
mod tree_builder;

pub fn execute(val: &str) -> String {
    let tokens = tokenizer::tokenize(val);
    let tree = tree_builder::build_tree(&tokens);
    let result = evaluator::eval(&tree);
    return result;
}
