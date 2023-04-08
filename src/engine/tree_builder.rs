
use crate::engine::tokenizer::{TagStatus, Token, TokenType};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node {
    pub token_type: TokenType,
    pub token_value: String,
    pub content: Option<Vec<Node>>,
}

pub fn build_tree(tokens: &Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens.get(i).unwrap().clone();
        let (new_i, node) = match token.tag_status {
            TagStatus::Open => tree(i, tokens, 1.0),
            TagStatus::Close => panic!("Unexpected close tag: {:?}", token),
            _ => (i + 1, Node {
                token_type:
                tokens.get(i).unwrap().clone().token_type,
                token_value: tokens.get(i).unwrap().clone().value,
                content: None,
            }),
        };
        i = new_i;
        nodes.push(node);
    }
    return nodes;
}

fn tree(current_index: usize, tokens: &Vec<Token>, open_tags: f32) -> (usize, Node) {
    let mut content_nodes = Vec::new();

    let mut i = current_index + 1;

    while i < tokens.len() {
        let mut curr_open_tags = open_tags;
        let token = tokens.get(i).unwrap().clone();

        if token.tag_status == TagStatus::Close &&
            token.token_type == tokens.get(current_index).unwrap().clone().token_type {
            i += 1;
            return (i, Node {
                token_type: tokens.get(current_index).unwrap().clone().token_type,
                token_value: tokens.get(current_index).unwrap().clone().value,
                content: Some(content_nodes),
            });
        }

        if token.tag_status == TagStatus::Open {
            let (new_index, contents) = tree(i, tokens, curr_open_tags + 1.0);
            i = new_index;
            content_nodes.push(contents);
            continue;
        }

        if token.tag_status == TagStatus::Close {
            curr_open_tags -= 1.0;
            if curr_open_tags < 0.0 {
                panic!("Unexpected close tag: {:?}", token);
            }
        }

        content_nodes.push(Node {
            token_type: token.token_type,
            token_value: token.value,
            content: None,
        });

        i += 1;
    }

    return (current_index + 1, Node {
        token_type: tokens.get(current_index).unwrap().clone().token_type,
        token_value: tokens.get(current_index).unwrap().clone().value,
        content: Some(content_nodes),
    });
}