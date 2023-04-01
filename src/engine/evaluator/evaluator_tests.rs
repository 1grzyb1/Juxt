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

#[test]
fn parse_content_and_function() {
    let nodes = vec![Node {
        token_type: TokenType::Script,
        token_value: "function test() { return 'test' }".to_string(),
        content: None,
    }, Node {
        token_type: TokenType::Content,
        token_value: "Hello World=".to_string(),
        content: None,
    }, Node {
        token_type: TokenType::Function,
        token_value: "test()".to_string(),
        content: None,
    }];
    let result = eval(&nodes);
    assert_eq!(result, "Hello World=test");
}