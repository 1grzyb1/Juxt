use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;
use uuid::Uuid;

#[cfg(test)]
mod tree_parser_tests;

fn parse_tree(nodes: &Vec<Node>) -> String {
    let mut replecments = Vec::new();
    let mut content = String::new();

    for node in nodes {
        match node.token_type {
            TokenType::Content => {
                content.push_str(&node.token_value);
            }
            TokenType::Function => {
                let id = Uuid::new_v4().to_string();
                content.push_str(&id);
                replecments.push(fn_replacment(&id, &node.token_value));
            }
            _ => println!("Not implemented yet"),
        }
    }

    let mut js = format!("let content = '{}'\n", content);
    for replecment in replecments {
        js.push_str(format!("{}\n", &replecment).as_str());
    }
    return format!("function execute(context) {{ \n {} \n return content \n }}", js);
}

fn fn_replacment(id: &str, replacment: &str) -> String {
    return format!("content = content.replace({}, {})", id, replacment);
}