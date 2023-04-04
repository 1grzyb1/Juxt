use js_sandbox::{Script};
use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;
use random_string::generate;

#[cfg(test)]
mod evaluator_tests;

pub fn eval(js_code: &str) -> String {
    println!("{}", js_code);
    let mut script = Script::from_string(&js_code).unwrap();

    let arg = 7;
    let result = script.call("execute", &arg).unwrap();

    return result;
}

pub fn generate_js(fn_name: &str, param: &str, nodes: &Vec<Node>, functions: Vec<String>) -> String {
    let mut functions = functions;
    let mut content = String::new();
    let mut scripts = String::new();

    for node in nodes {
        match node.token_type {
            TokenType::Script => {
                scripts.push_str(&script_replacement(node));
            }
            TokenType::Content => {
                content.push_str(&node.token_value);
            }
            TokenType::Function => {
                content.push_str(&format!("${{{}}}", node.token_value));
            }
            TokenType::Each => {
                let (map, map_function) = each_replacement(node);
                functions.push(map_function);
                content.push_str(&format!("${{{}}}", map));
            }
            _ => println!("Not implemented yet"),
        }
    }


    let mut js = format!("{} \n let content = `{}`\n", scripts, content);
    for function in functions {
        js.push_str(format!("{}\n", &function).as_str());
    }
    return format!("function {}({}) {{ \n {} \n return content \n }}", fn_name, param, js);
}

fn script_replacement(node: &Node) -> String {
    let mut content = Vec::new();
    for content_node in node.content.as_ref().unwrap() {
        if content_node.token_type != TokenType::Content {
            panic!("Script tag can only contain content");
        }

        content.push(content_node.token_value.clone());
    }
    return content.join("");
}

fn fn_replacement(id: &str, replacment: &str) -> String {
    return format!("content = content.replace(`{}`, {})", id, replacment);
}

fn each_replacement(node: &Node) -> (String, String) {
    let each: Vec<&str> = node.token_value.split(" in ").collect();

    let map_function_name = generate(6, "abcdefghijklmnopqrstuvwxyz");
    let map_function = generate_js(&map_function_name.to_string(), each[0], &node.content.as_ref().unwrap(), Vec::new());
    let map = format!("{}.map({} => {}({})).join('')", each[1], each[0], map_function_name, each[0]);
    return (map, map_function);
}