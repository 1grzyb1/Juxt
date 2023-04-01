use super::*;

#[test]
fn build_one_element_tree() {
    let tokens = vec![Token {
        value: String::from(""),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    }];
    let nodes = build_tree(&tokens);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].token_type, TokenType::Content);
    assert_eq!(nodes[0].token_value, "");
    assert_eq!(nodes[0].content, None);
}

#[test]
fn build_nested_tree() {
    let tokens = vec![Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from("bla"),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    },Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Close,
    }
    ];
    let nodes = build_tree(&tokens);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].token_type, TokenType::Each);
    assert_eq!(nodes[0].token_value, "");

    let content = nodes[0].content.as_ref().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0].token_type, TokenType::Content);
    assert_eq!(content[0].token_value, "bla");
}

#[test]
fn build_nested_tree_with_two_roots() {
    let tokens = vec![Token {
        value: String::from("bla"),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from("bla"),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    },Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Close,
    }
    ];
    let nodes = build_tree(&tokens);
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].token_type, TokenType::Content);
    assert_eq!(nodes[0].token_value, "bla");

    assert_eq!(nodes[1].token_type, TokenType::Each);
    assert_eq!(nodes[1].token_value, "");

    let content = nodes[1].content.as_ref().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0].token_type, TokenType::Content);
    assert_eq!(content[0].token_value, "bla");
}

#[test]
fn build_double_nested_tree_with_two_roots() {
    let tokens = vec![Token {
        value: String::from("bla"),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from("bla"),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Close,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Close,
    }
    ];
    let nodes = build_tree(&tokens);
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].token_type, TokenType::Content);
    assert_eq!(nodes[0].token_value, "bla");

    assert_eq!(nodes[1].token_type, TokenType::Each);
    assert_eq!(nodes[1].token_value, "");

    let content = nodes[1].content.as_ref().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0].token_type, TokenType::Each);

    let nested_content = content[0].content.as_ref().unwrap();
    assert_eq!(nested_content.len(), 1);
    assert_eq!(nested_content[0].token_type, TokenType::Content);
}

#[test]
fn build_double_nested_tree_without_close_tag_with_two_open() {
    let tokens = vec![Token {
        value: String::from("bla"),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Function,
        tag_status: TagStatus::Open,
    },Token {
        value: String::from(""),
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Close,
    }
    ];
    let nodes = build_tree(&tokens);
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].token_type, TokenType::Content);
    assert_eq!(nodes[0].token_value, "bla");

    assert_eq!(nodes[1].token_type, TokenType::Each);
    assert_eq!(nodes[1].token_value, "");

    let content = nodes[1].content.as_ref().unwrap();
    assert_eq!(content.len(), 2);
    assert_eq!(content[0].token_type, TokenType::Function);
    assert_eq!(content[1].token_type, TokenType::Content);
}

#[test]
fn build_double_nested_tree_without_close_tag_with_two_roots() {
    let tokens = vec![Token {
        value: String::from("import"),
        token_type: TokenType::Import,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Function,
        tag_status: TagStatus::Open,
    },Token {
        value: String::from(""),
        token_type: TokenType::Function,
        tag_status: TagStatus::Open,
    }, Token {
        value: String::from(""),
        token_type: TokenType::Each,
        tag_status: TagStatus::Close,
    }
    ];
    let nodes = build_tree(&tokens);
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].token_type, TokenType::Import);
    assert_eq!(nodes[0].token_value, "import");

    assert_eq!(nodes[1].token_type, TokenType::Each);
    assert_eq!(nodes[1].token_value, "");

    let content = nodes[1].content.as_ref().unwrap();
    assert_eq!(content.len(), 2);
    assert_eq!(content[0].token_type, TokenType::Function);
    assert_eq!(content[1].token_type, TokenType::Function);
}