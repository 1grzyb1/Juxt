use std::error::Error;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Import,
    Script,
    Each,
    Content,
    If,
    Else,
    Comment
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagStatus {
    Open,
    Close,
    Undefined,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub tag_status: TagStatus,
}

pub fn tokenize(val: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut pointer = 0;
    while pointer < val.len() {
        let (new_pointer, token) = next_token(pointer, val)?;
        tokens.push(token);
        pointer = new_pointer;
    }

    let concat = concat_content(&tokens);
    return Ok(clear_whitespace(concat));
}

fn clear_whitespace(tokens: Vec<Token>) -> Vec<Token> {
    let mut cleared: Vec<Token> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        if should_remove_leading_newline(i, token, &tokens) {
            let mut value = token.value.clone();
            value = remove_leading_newline(&value);
            cleared.push(Token {
                value,
                token_type: token.token_type.clone(),
                tag_status: token.tag_status.clone(),
            });
            continue;
        }

        if i >= tokens.len() - 1 || should_not_remove_trailing_whitespace(i, token, &tokens) {
            cleared.push(token.clone());
            continue;
        }

        let value = remove_trailing_whitespace(&token.value);

        cleared.push(Token {
            value,
            token_type: token.token_type.clone(),
            tag_status: token.tag_status.clone(),
        });
    }

    cleared
}

fn should_remove_leading_newline(i: usize, token: &Token, tokens: &[Token]) -> bool {
    i > 0
        && token.token_type == TokenType::Content
        && tokens[i - 1].token_type != TokenType::Content
}

fn should_not_remove_trailing_whitespace(i: usize, token: &Token, tokens: &[Token]) -> bool {
    token.token_type != TokenType::Content && tokens[i + 1].token_type != TokenType::Content
}

fn remove_leading_newline(value: &str) -> String {
    if value.starts_with('\n') {
        value[1..].to_string()
    } else {
        value.to_string()
    }
}

fn remove_trailing_whitespace(value: &str) -> String {
    let trimmed = value.trim_end_matches(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n');
    trimmed.to_string()
}

fn concat_content(tokens: &Vec<Token>) -> Vec<Token> {
    let mut concatinated: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens.get(i).unwrap().clone();
        if token.token_type != TokenType::Content {
            concatinated.push(token);
            i += 1;
            continue;
        }

        let mut concat_value = Vec::new();
        concat_value.push(token.value);
        i += 1;
        while i < tokens.len() && tokens.get(i).unwrap().token_type == TokenType::Content {
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

fn next_token(pointer: usize, val: &str) -> Result<(usize, Token), Box<dyn Error>> {
    let mut pointer = pointer;
    if get_char_at(pointer, val)? == '{' {
        let (new_pointer, token) = match_token(pointer, val)?;
        if !matches!(token.token_type, TokenType::Content) {
            return Ok((new_pointer, token));
        }
    }

    if val[pointer..].starts_with("//") {
        let mut end_of_line = val[pointer..].find('\n').unwrap_or(val.len());

        if end_of_line != val.len() {
            end_of_line = end_of_line + pointer;
        }

        return Ok((
            end_of_line,
            Token {
                value: val[pointer+2..end_of_line].to_string(),
                token_type: TokenType::Comment,
                tag_status: TagStatus::Undefined,
            }
        ));
    }

    let mut content = String::from(get_char_at(pointer, val)?);
    pointer = pointer + 1;
    while pointer < val.len() && get_char_at(pointer, val)? != '{' && !val[pointer..].starts_with("//"){
        content = vec![content, String::from(get_char_at(pointer, val)?)].join("");
        pointer = pointer + 1;
    }
    return Ok((
        pointer,
        Token {
            value: content,
            token_type: TokenType::Content,
            tag_status: TagStatus::Undefined,
        },
    ));
}

fn match_token(pointer: usize, val: &str) -> Result<(usize, Token), Box<dyn Error>> {
    let mut pointer = pointer + 1;
    if get_char_at(pointer, val)? != '#' && get_char_at(pointer, val)? != '/' {
        return Ok((
            pointer,
            Token {
                value: String::from("{"),
                token_type: TokenType::Content,
                tag_status: TagStatus::Undefined,
            },
        ));
    }
    let tag_status = match get_char_at(pointer, val)? {
        '#' => TagStatus::Open,
        '/' => TagStatus::Close,
        _ => TagStatus::Undefined,
    };

    pointer = pointer + 1;
    pointer = skip_whitespace(pointer, val)?;
    let (new_pointer, type_token) = read_token_type(pointer, val)?;
    pointer = new_pointer;
    pointer = skip_whitespace(pointer, val)?;
    let (pointer, content) = read_content(pointer, val)?;

    return Ok((
        pointer + 1,
        Token {
            value: String::from(content),
            token_type: match_type(type_token),
            tag_status,
        },
    ));
}

fn match_type(type_token: &str) -> TokenType {
    match type_token.trim() {
        "import" => TokenType::Import,
        "script" => TokenType::Script,
        "each" => TokenType::Each,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        _ => TokenType::Content,
    }
}

fn read_content(beginning: usize, val: &str) -> Result<(usize, &str), Box<dyn Error>> {
    let mut pointer = beginning;
    while get_char_at(pointer, val)? != '}' {
        pointer = pointer + 1;
    }
    return Ok((pointer, &val[beginning..pointer]));
}

fn read_token_type(beginning: usize, val: &str) -> Result<(usize, &str), Box<dyn Error>> {
    let mut pointer = beginning;
    while get_char_at(pointer, val)? != ' ' && get_char_at(pointer, val)? != '}' {
        pointer = pointer + 1;
    }
    return Ok((pointer, &val[beginning..pointer]));
}

fn skip_whitespace(pointer: usize, val: &str) -> Result<usize, Box<dyn Error>> {
    let mut pointer = pointer;
    while get_char_at(pointer, val)? == ' ' {
        pointer = pointer + 1;
    }
    return Ok(pointer);
}

fn get_char_at(pointer: usize, val: &str) -> Result<char, Box<dyn Error>> {
    val.chars()
        .nth(pointer)
        .ok_or(format!("There is no char at given index {}, {}", pointer, val).into())
}
