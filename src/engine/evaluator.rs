use js_sandbox::{Script};
use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;
use random_string::generate;
use std::error::Error;

pub fn eval(js_code: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", js_code);
    let mut script = Script::from_string(&js_code)?;

    let arg = 7;
    let result = script.call("execute", &arg)?;

    Ok(result)
}

pub fn generate_js(fn_name: &str, param: &str, nodes: &Vec<Node>, functions: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut functions = functions;
    let mut content = String::new();
    let mut scripts = String::new();

    for (i, node) in nodes.iter().enumerate() {
        match node.token_type {
            TokenType::Script => {
                scripts.push_str(&script_replacement(node)?);
            }
            TokenType::Content => {
                content.push_str(&node.token_value);
            }
            TokenType::Each => {
                let (map, map_function) = each_replacement(node)?;
                functions.push(map_function);
                content.push_str(&format!("${{{}}}", map));
            }
            TokenType::If => {
                let (condition, condition_function, else_function) = if_replacement(i, nodes)?;
                functions.push(condition_function);
                functions.push(else_function);
                content.push_str(&format!("${{{}}}", condition));
            }
            TokenType::Else => {
                // Ignore handled in if match
            }
            _ => println!("Not implemented yet"),
        }
    }

    let mut js = format!("{} \n let content = `{}`\n", scripts, content);
    for function in functions {
        js.push_str(format!("{}\n", &function).as_str());
    }
    Ok(format!("function {}({}) {{ \n {} \n return content \n }}", fn_name, param, js))
}

fn if_replacement(i: usize, nodes: &Vec<Node>) -> Result<(String, String, String), Box<dyn Error>> {
    let node = nodes.get(i).ok_or("Node not found")?;

    let else_function_name = generate_fn_name();
    let mut else_function = generate_js(&else_function_name, "", &Vec::new(), Vec::new())?;

    if (i + 1) < nodes.len() {
        let next_node = nodes.get(i + 1).ok_or("Next node not found")?;
        else_function = generate_js(&else_function_name, "", &next_node.content.as_ref().unwrap(), Vec::new())?;
    }

    let if_function_name = generate_fn_name();
    let if_function = generate_js(&if_function_name, "", &node.content.as_ref().unwrap(), Vec::new())?;
    let if_value = format!("{} ? {}() : {}()", node.token_value, if_function_name, else_function_name);
    Ok((if_value, if_function, else_function))
}

fn script_replacement(node: &Node) -> Result<String, Box<dyn Error>> {
    let mut content = Vec::new();
    for content_node in node.content.as_ref().ok_or("Node content not found")? {
        if content_node.token_type != TokenType::Content {
            return Err("Script tag can only contain content".into());
        }

        content.push(content_node.token_value.clone());
    }
    Ok(content.join(""))
}

fn each_replacement(node: &Node) -> Result<(String, String), Box<dyn Error>> {
    let each: Vec<&str> = node.token_value.split(" in ").collect();

    let map_function_name = generate_fn_name();
    let map_function = generate_js(&map_function_name.to_string(), each[0], &node.content.as_ref().unwrap(), Vec::new())?;
    let map = format!("{}.map({} => {}({})).join('')", each[1], each[0], map_function_name, each[0]);
    return Ok((map, map_function));
}

fn generate_fn_name() -> String {
    return generate(6, "abcdefghijklmnopqrstuvwxyz");
}