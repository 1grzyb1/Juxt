use super::*;

#[test]
fn parse_content() {
    let nodes = vec![Node {
        token_type: TokenType::Content,
        token_value: "Hello World".to_string(),
        content: None,
    }];
    let result = eval(&nodes);
    assert_eq!(result, "Hello World");
}

// #[test]
fn parse_content_and_function() {
    let nodes = vec![Node {
        token_type: TokenType::Content,
        token_value: "Hello World=".to_string(),
        content: None,
    },Node {
        token_type: TokenType::Function,
        token_value: "function()".to_string(),
        content: None,
    }];
    let result = parse_tree(&nodes);
    assert_eq!(result, "function execute(context) { \n let content = 'Hello World'\n \n return content \n }");
}