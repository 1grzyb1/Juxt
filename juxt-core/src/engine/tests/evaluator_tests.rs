use crate::engine::evaluator::{eval, generate_js};
use crate::engine::tokenizer::TokenType;
use crate::engine::tree_builder::Node;

#[test]
fn parse_content() {
    let nodes = vec![Node {
        token_type: TokenType::Content,
        token_value: "Hello World".to_string(),
        content: None,
    }];
    let result = eval(
        &generate_js("execute", "context", &nodes, Vec::new()).unwrap(),
        String::new(),
    )
    .unwrap();
    assert_eq!(result, "Hello World");
}

#[test]
fn parse_content_and_function() {
    let nodes = vec![
        Node {
            token_type: TokenType::Script,
            token_value: "".to_string(),
            content: Option::from(vec![Node {
                token_type: TokenType::Content,
                token_value: "function test() { return 'test' }".to_string(),
                content: None,
            }]),
        },
        Node {
            token_type: TokenType::Content,
            token_value: "Hello World=${test()}".to_string(),
            content: None,
        },
    ];
    let result = eval(
        &generate_js("execute", "context", &nodes, Vec::new()).unwrap(),
        String::new(),
    )
    .unwrap();
    assert_eq!(result, "Hello World=test");
}

#[test]
fn parse_each() {
    let nodes = vec![Node {
        token_type: TokenType::Each,
        token_value: "n in [1, 2, 3]".to_string(),
        content: Option::from(vec![Node {
            token_type: TokenType::Content,
            token_value: "test".to_string(),
            content: None,
        }]),
    }];
    let result = eval(
        &generate_js("execute", "context", &nodes, Vec::new()).unwrap(),
        String::new(),
    )
    .unwrap();
    assert_eq!(result, "testtesttest");
}

#[test]
fn parse_each_with_function() {
    let nodes = vec![
        Node {
            token_type: TokenType::Script,
            token_value: "".to_string(),
            content: Option::from(vec![Node {
                token_type: TokenType::Content,
                token_value: "function test(n) { return 'test' + n }".to_string(),
                content: None,
            }]),
        },
        Node {
            token_type: TokenType::Each,
            token_value: "n in [1, 2, 3]".to_string(),
            content: Option::from(vec![Node {
                token_type: TokenType::Content,
                token_value: "${test(n)}".to_string(),
                content: None,
            }]),
        },
    ];
    let result = eval(
        &generate_js("execute", "context", &nodes, Vec::new()).unwrap(),
        String::new(),
    )
    .unwrap();
    assert_eq!(result, "test1test2test3");
}
