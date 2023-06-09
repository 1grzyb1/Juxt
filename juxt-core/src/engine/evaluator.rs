use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;
use debug_print::debug_println;
use js_sandbox::Script;
use random_string::generate;
use std::error::Error;

pub fn eval(js_code: &str, context: String) -> Result<String, Box<dyn Error>> {
    debug_println!("context: {} \n {}", context, js_code);
    let mut script = Script::from_string(&js_code)?;

    let mut args = String::from("{}");

    if !context.is_empty() {
        args = context;
    }

    let result = script.call("execute", &args)?;

    Ok(result)
}

pub struct Import {
    pub file_name: String,
    pub name: String,
    pub extension: String,
    pub orginial_value: String,
    pub content: Vec<Node>,
}

pub struct Compiler {
    pub imports: Vec<Import>,
}

impl Compiler {
    pub fn generate_js(
        &self,
        fn_name: &str,
        param: &str,
        nodes: &Vec<Node>,
    ) -> Result<String, Box<dyn Error>> {
        let mut functions = Vec::new();
        let mut content = String::new();
        let mut scripts = String::new();

        for (i, node) in nodes.iter().enumerate() {
            match node.token_type {
                TokenType::Import => self.handle_import(&mut functions, node)?,
                TokenType::Script => scripts.push_str(&self.script_replacement(node)?),
                TokenType::Content => content.push_str(&node.token_value),
                TokenType::Each => self.handle_each(&mut functions, &mut content, node)?,
                TokenType::If => self.handle_if(&mut functions, &mut content, i, nodes)?,
                TokenType::Else => { /* Ignore, handled in if */ }
                TokenType::Comment => { /* Ignore */ }
            }
        }

        let mut js = String::new();
        if !param.is_empty() {
            js.push_str(&format!(
                "try {{ {} = JSON.parse({}) }} catch (error) {{}}\n",
                param, param
            ));
        }

        js.push_str(&format!(
            "
       {} \n let content = `{}`\n",
            scripts, content
        ));

        for function in functions {
            js.push_str(&format!("{}\n", &function));
        }

        Ok(format!(
            "function {}({}) {{ \n {} \n return content \n }}",
            fn_name, param, js
        ))
    }

    fn handle_import(
        &self,
        functions: &mut Vec<String>,
        node: &Node,
    ) -> Result<(), Box<dyn Error>> {
        let import_name = node.token_value.clone();
        let import = self.imports.iter().find(|i| i.file_name == import_name);
        match import {
            Some(import) => match import.extension.as_str() {
                "juxt" => {
                    let js = self.generate_js(&import.name, "context", &import.content)?;
                    functions.push(js);
                    Ok(())
                }
                "js" => {
                    functions.push(import.orginial_value.clone());
                    Ok(())
                }
                _ => return Err(format!("Unknown extension {}", import.extension).into()),
            },
            None => Err(format!("Import {} not found", import_name).into()),
        }
    }

    fn handle_each(
        &self,
        functions: &mut Vec<String>,
        content: &mut String,
        node: &Node,
    ) -> Result<(), Box<dyn Error>> {
        let (map, map_function) = self.each_replacement(node)?;
        functions.push(map_function);
        content.push_str(&format!("${{{}}}", map));
        Ok(())
    }

    fn handle_if(
        &self,
        functions: &mut Vec<String>,
        content: &mut String,
        index: usize,
        nodes: &Vec<Node>,
    ) -> Result<(), Box<dyn Error>> {
        let (condition, condition_function, else_function) = self.if_replacement(index, nodes)?;
        functions.push(condition_function);
        functions.push(else_function);
        content.push_str(&format!("${{{}}}", condition));
        Ok(())
    }

    fn if_replacement(
        &self,
        i: usize,
        nodes: &Vec<Node>,
    ) -> Result<(String, String, String), Box<dyn Error>> {
        let node = nodes.get(i).ok_or("Node not found")?;

        let else_function_name = self.generate_fn_name();
        let mut else_function = self.generate_js(&else_function_name, "", &Vec::new())?;

        if (i + 1) < nodes.len() {
            let next_node = nodes.get(i + 1).ok_or("Next node not found")?;
            if next_node.token_type == TokenType::Else {
                else_function = self.generate_js(
                    &else_function_name,
                    "",
                    &next_node.content.as_ref().unwrap(),
                )?;
            }
        }

        let if_function_name = self.generate_fn_name();
        let if_function =
            self.generate_js(&if_function_name, "", &node.content.as_ref().unwrap())?;
        let if_value = format!(
            "{} ? {}() : {}()",
            node.token_value, if_function_name, else_function_name
        );
        Ok((if_value, if_function, else_function))
    }

    fn script_replacement(&self, node: &Node) -> Result<String, Box<dyn Error>> {
        let mut content = Vec::new();
        for content_node in node.content.as_ref().ok_or("Node content not found")? {
            if content_node.token_type != TokenType::Content {
                return Err("Script tag can only contain content".into());
            }

            content.push(content_node.token_value.clone());
        }
        Ok(content.join(""))
    }

    fn each_replacement(&self, node: &Node) -> Result<(String, String), Box<dyn Error>> {
        let each: Vec<&str> = node.token_value.split(" in ").collect();

        let map_function_name = self.generate_fn_name();
        let map_function = self.generate_js(
            &map_function_name.to_string(),
            each[0],
            &node.content.as_ref().unwrap(),
        )?;
        let map = format!(
            "{}.map({} => {}({})).join('')",
            each[1], each[0], map_function_name, each[0]
        );
        return Ok((map, map_function));
    }

    fn generate_fn_name(&self) -> String {
        return generate(6, "abcdefghijklmnopqrstuvwxyz");
    }
}
