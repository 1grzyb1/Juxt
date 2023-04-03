use super::*;

#[test]
fn parse_content() {
    let nodes = vec![Node {
        token_type: TokenType::Content,
        token_value: "Hello World".to_string(),
        content: None,
    }];
    let js_code = generate_js("execute", "context", &nodes, &mock_provider);
    let result = eval(&js_code);
    assert_eq!(result, "Hello World");
}

#[test]
fn parse_content_and_function() {
    let nodes = vec![Node {
        token_type: TokenType::Script,
        token_value: "".to_string(),
        content: Option::from(vec![Node {
            token_type: TokenType::Content,
            token_value: "function test() { return 'test' }".to_string(),
            content: None,
        }]),
    }, Node {
        token_type: TokenType::Content,
        token_value: "Hello World=".to_string(),
        content: None,
    }, Node {
        token_type: TokenType::Function,
        token_value: "test()".to_string(),
        content: None,
    }];
    let js_code = generate_js("execute", "context", &nodes, &mock_provider);
    let result = eval(&js_code);
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
    let js_code = generate_js("execute", "context", &nodes, &mock_provider);
    let result = eval(&js_code);
    assert_eq!(result, "testtesttest");
}

#[test]
fn parse_each_with_function() {
    let nodes = vec![Node {
        token_type: TokenType::Script,
        token_value: "".to_string(),
        content: Option::from(vec![Node {
            token_type: TokenType::Content,
            token_value: "function test(n) { return 'test' + n }".to_string(),
            content: None,
        }]),
    }, Node {
        token_type: TokenType::Each,
        token_value: "n in [1, 2, 3]".to_string(),
        content: Option::from(vec![Node {
            token_type: TokenType::Function,
            token_value: "test(n)".to_string(),
            content: None,
        }]),
    }];
    let js_code = generate_js("execute", "context", &nodes, &mock_provider);
    let result = eval(&js_code);
    assert_eq!(result, "test1test2test3");
}

#[test]
fn parse_import_component() {
    let nodes = vec![Node {
            token_type: TokenType::Import,
            token_value: "component.juxt".to_string(),
            content: None,
        }, Node {
        token_type: TokenType::Content,
        token_value: "Hello World=".to_string(),
        content: None,
    }, Node {
        token_type: TokenType::Function,
        token_value: "component()".to_string(),
        content: None,
    }];
    let js_code = generate_js("execute", "context", &nodes, &mock_provider);
    let result = eval(&js_code);
    assert_eq!(result, "Hello World=test");
}

fn mock_provider(template_name: &str, _: &str) -> String {
    if template_name == "component.juxt" {
        return "function component() { return 'test' }".to_string();
    }
    return "".to_string();
}