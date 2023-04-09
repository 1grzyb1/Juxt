use clap::Parser;
use std::error::Error;
use std::fs;
use std::io::Read;
use walkdir::WalkDir;

use juxt_core::compile_and_execute;
use juxt_core::engine::Juxt;

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[arg(short, long, default_value_t = String::from(""), help = "Path from which template will be read")]
    path: String,

    #[arg(short, long, default_value_t = String::from("main.juxt"), help = "Main template name")]
    main: String,

    #[arg(short, long, default_value_t = String::from("{}"), help = "Context in JSON format")]
    context: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    println!("{:?}", args);

    let files = read_files_recursive(&args.path)?;
    let (main, dependencies) = get_flux_files(files, &args.main)?;

    let result = compile_and_execute(main, dependencies, args.context.to_string())?;

    println!("{}", result);
    Ok(())
}

fn get_flux_files(
    files: Vec<Juxt>,
    main_name: &String,
) -> Result<(Juxt, Vec<Juxt>), Box<dyn Error>> {
    let mut main = Juxt {
        name: String::new(),
        template: String::new(),
    };
    let mut dependencies = Vec::new();

    for file in files {
        if file.name == *main_name {
            main = file;
            continue;
        }
        dependencies.push(file);
    }

    if main.name.is_empty() {
        return Err("Main file not found".into());
    }

    Ok((main, dependencies))
}

fn read_files_recursive(directory: &str) -> Result<Vec<Juxt>, Box<dyn Error>> {
    let mut juxts = Vec::new();
    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let mut file_content = String::new();
        let mut file = fs::File::open(path)?;
        file.read_to_string(&mut file_content)?;

        let file_name = path
            .to_str()
            .unwrap()
            .strip_prefix(&(directory.to_string() + "/"))
            .unwrap();

        juxts.push(Juxt {
            name: file_name.to_string(),
            template: file_content,
        });
    }

    Ok(juxts)
}
