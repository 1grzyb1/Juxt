use js_sandbox::{Script, AnyError};
use uuid::Uuid;
use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;

#[cfg(test)]
mod evaluator_tests;

pub fn eval(nodes: &Vec<Node>) -> String {
    let js_code = parse_tree(nodes);
    let mut script = Script::from_string(&js_code).unwrap();

    let arg = 7;
    let result = script.call("execute", &arg).unwrap();

    return result;
}

fn parse_tree(nodes: &Vec<Node>) -> String {
    let mut replecments = Vec::new();
    let mut content = String::new();
    let mut scripts = String::new();

    for node in nodes {
        match node.token_type {
            TokenType::Script => {
                scripts.push_str(&node.token_value);
            }
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


    let mut js = format!("{} \n let content = '{}'\n", scripts, content);
    for replecment in replecments {
        js.push_str(format!("{}\n", &replecment).as_str());
    }
    return format!("function execute(context) {{ \n {} \n return content \n }}", js);
}

fn fn_replacment(id: &str, replacment: &str) -> String {
    return format!("content = content.replace(`{}`, {})", id, replacment);
}