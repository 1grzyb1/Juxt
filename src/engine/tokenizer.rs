
#[cfg(test)]
mod tokenizer_tests;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum TokenType {
    Import,
    Script,
    Each,
    Function,
    Content,
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum TagStatus {
    Open,
    Close,
    Undefined,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
    value: String,
    token_type: TokenType,
    tag_status: TagStatus,
}

pub fn tokenize(val: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut pointer = 0;
    while pointer < val.len() {
        let (new_pointer, token) = next_token(pointer, val);
        tokens.push(token);
        pointer = new_pointer;
    }

    return concat_content(&tokens);
}

fn concat_content(tokens: &Vec<Token>) -> Vec<Token> {
    let mut concatinated: Vec<Token> = Vec::new();
    let mut i = 0;
    while  i < tokens.len() {
        let token = tokens.get(i).unwrap().clone();
        if token.token_type != TokenType::Content {
            concatinated.push(token);
            i += 1;
            continue;
        }

        let mut concat_value = Vec::new();
        concat_value.push(token.value);
        i += 1;
        while i < tokens.len() && tokens.get(i).unwrap().token_type == TokenType::Content  {
            concat_value.push(tokens.get(i).unwrap().value.clone());
            i += 1;
        }

        concatinated.push(Token {
            value: concat_value.join(""),
            token_type: TokenType::Content,
            tag_status: TagStatus::Undefined,
        })
    }
    return concatinated;
}

fn next_token(pointer: usize, val: &str) -> (usize, Token) {
    let mut pointer = pointer;
    if get_char_at(pointer, val) == '{' {
        let (new_pointer, token) = match_token(pointer, val);
        if !matches!(token.token_type, TokenType::Content) {
            return (new_pointer, token);
        }
    }

    let mut content = String::from(get_char_at(pointer, val));
    pointer = pointer + 1;
    while pointer < val.len() && get_char_at(pointer, val) != '{' {
        content = vec![content, String::from(get_char_at(pointer, val))].join("");
        pointer = pointer + 1;
    }
    return (pointer, Token {
        value: content,
        token_type: TokenType::Content,
        tag_status: TagStatus::Undefined,
    });
}

fn match_token(pointer: usize, val: &str) -> (usize, Token) {
    let mut pointer = pointer + 1;
    if get_char_at(pointer, val) != '#' && get_char_at(pointer, val) != '/' {
        return (pointer, Token {
            value: String::from("{"),
            token_type: TokenType::Content,
            tag_status: TagStatus::Undefined,
        });
    }
    let tag_status = match get_char_at(pointer, val) {
        '#' => TagStatus::Open,
        '/' => TagStatus::Close,
        _ => TagStatus::Undefined,
    };

    pointer = pointer + 1;
    pointer = skip_whitespace(pointer, val);
    let (new_pointer, type_token) = read_token_type(pointer, val);
    pointer = new_pointer;
    pointer = skip_whitespace(pointer, val);
    let (pointer, content) = read_content(pointer, val);

    return (pointer + 1, Token {
        value: String::from(content),
        token_type: match_type(type_token),
        tag_status,
    });
}

fn match_type(type_token: &str) -> TokenType {
    match type_token.trim() {
        "import" => TokenType::Import,
        "script" => TokenType::Script,
        "each" => TokenType::Each,
        "fn" => TokenType::Function,
        _ => TokenType::Content,
    }
}

fn read_content(beginning: usize, val: &str) -> (usize, &str) {
    let mut pointer = beginning;
    while get_char_at(pointer, val) != '}' {
        pointer = pointer + 1;
    }
    return (pointer, &val[beginning..pointer]);
}

fn read_token_type(beginning: usize, val: &str) -> (usize, &str) {
    let mut pointer = beginning;
    while get_char_at(pointer, val) != ' ' && get_char_at(pointer, val) != '}' {
        pointer = pointer + 1;
    }
    return (pointer, &val[beginning..pointer]);
}

fn skip_whitespace(pointer: usize, val: &str) -> usize {
    let mut pointer = pointer;
    while get_char_at(pointer, val) == ' ' {
        pointer = pointer + 1;
    }
    return pointer;
}


fn get_char_at(pointer: usize, val: &str) -> char {
    return val.chars().nth(pointer).unwrap();
}

