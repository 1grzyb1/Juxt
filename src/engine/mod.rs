mod tokenizer;

pub fn execute(val: &str) -> String {
    let _tokens = tokenizer::tokenize(val);
    return String::from("Hello, world!");
}