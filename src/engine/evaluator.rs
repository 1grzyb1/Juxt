use js_sandbox::{Script, AnyError};
use uuid::Uuid;
use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;
use random_string::generate;

#[cfg(test)]
mod evaluator_tests;

pub fn eval(nodes: &Vec<Node>) -> String {
    let js_code = generate_js("execute", "context", nodes);
    println!("{}", js_code);
    let mut script = Script::from_string(&js_code).unwrap();

    let arg = 7;
    let result = script.call("execute", &arg).unwrap();

    return result;
}

fn generate_js(fn_name: &str, param: &str, nodes: &Vec<Node>) -> String {
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
                replecments.push(fn_replacement(&id, &node.token_value));
            }
            TokenType::Each => {
                let id = Uuid::new_v4().to_string();
                content.push_str(&id);
                replecments.push(each_replacement(&id, &node));
            }
            _ => println!("Not implemented yet"),
        }
    }


    let mut js = format!("{} \n let content = '{}'\n", scripts, content);
    for replecment in replecments {
        js.push_str(format!("{}\n", &replecment).as_str());
    }
    return format!("function {}({}) {{ \n {} \n return content \n }}", fn_name, param, js);
}

fn fn_replacement(id: &str, replacment: &str) -> String {
    return format!("content = content.replace(`{}`, {})", id, replacment);
}

fn each_replacement(id: &String, node: &Node) -> String {
    let each: Vec<&str> = node.token_value.split(" in ").collect();

    let map_function_name = generate(6, "abcdefghijklmnopqrstuvwxyz");
    let mut map_function = generate_js(&map_function_name.to_string(), each[0], &node.content.as_ref().unwrap());
    return format!("{} \n content = content.replace(`{}`, {})", map_function, id, format!("{}.map({} => {}({})).join('')", each[1], each[0], map_function_name, each[0]));
}