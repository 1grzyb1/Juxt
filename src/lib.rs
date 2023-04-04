use crate::engine::{compile, execute, Juxt};

pub mod engine;


pub fn compile_and_execute(main: Juxt, dependencie: Vec<Juxt>) -> String {
    let compiled = compile(main, dependencie);
    return execute(&compiled);
}