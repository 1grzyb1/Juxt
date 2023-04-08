use std::error::Error;

mod tokenizer;
mod evaluator;
mod tree_builder;

#[cfg(test)]
mod tests {
    mod evaluator_tests;
    mod tokenizer_tests;
    mod tree_builder_tests;
}

pub struct Juxt {
    pub name: String,
    pub template: String,
}

pub fn execute(js_code: &str) -> Result<String, Box<dyn Error>> {
    return evaluator::eval(js_code);
}

pub fn compile(main: Juxt, dependencie: Vec<Juxt>) -> Result<String, Box<dyn Error>> {
    let tokens = tokenizer::tokenize(&main.template)?;
    let tree = tree_builder::build_tree(&tokens);
    let compiled_dependencies = compile_dependencies(&dependencie)?;
    let compiled_main = evaluator::generate_js("execute", "context", &tree, compiled_dependencies);
    return compiled_main;
}

fn compile_dependencies(juxts: &Vec<Juxt>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut compiled = Vec::new();
    for juxt in juxts {
        let tokens = tokenizer::tokenize(&juxt.template)?;
        let tree = tree_builder::build_tree(&tokens);
        let (extension, name) = split_name(&juxt.name)?;
        let compiled_juxt= match extension {
            "js" => Ok(juxt.template.clone()),
            "juxt" => evaluator::generate_js(name, "context", &tree, Vec::new()),
            _ => Err("Script tag can only contain content".into())
        }?;

        compiled.push(compiled_juxt);
    }
    return Ok(compiled);
}

fn split_name(juxt_name: &String) -> Result<(&str, &str), Box<dyn Error>> {
    let split: Vec<&str> = juxt_name.split(".").map(|s| s).collect();
    let extension = split.last().ok_or("No extension found")?;
    let name = split.get(0).ok_or("No name found")?;
    Ok((*extension, *name))
}
