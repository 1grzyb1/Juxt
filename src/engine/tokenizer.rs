#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    Import,
    Script,
    Each,
    Function,
    Content,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TagStatus {
    Open,
    Close,
    Undefined,
}

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
    return tokens;
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

#[test]
fn should_tokenize_content() {
    let tokens = tokenize("some test");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "some test");
    assert_eq!(tokens[0].token_type, TokenType::Content);
    assert_eq!(tokens[0].tag_status, TagStatus::Undefined);
}

#[test]
fn should_tokenize_import() {
    let tokens = tokenize("{#import component.flux}");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "component.flux");
    assert_eq!(tokens[0].token_type, TokenType::Import);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);
}

#[test]
fn should_tokenize_import_and_content() {
    let tokens = tokenize("{#import component.flux} blbablb");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].value, "component.flux");
    assert_eq!(tokens[0].token_type, TokenType::Import);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    assert_eq!(tokens[1].value, " blbablb");
    assert_eq!(tokens[1].token_type, TokenType::Content);
    assert_eq!(tokens[1].tag_status, TagStatus::Undefined);
}

#[test]
fn should_tokenize_script_and_content() {
    let tokens = tokenize("{#script}");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[0].token_type, TokenType::Script);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    let tokens = tokenize("{# script  }");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[0].token_type, TokenType::Script);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);
}

#[test]
fn should_tokenize_open_cloased_and_content() {
    let tokens = tokenize("{#script} bigos bigos {/script}");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[0].token_type, TokenType::Script);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    assert_eq!(tokens[1].value, " bigos bigos ");
    assert_eq!(tokens[1].token_type, TokenType::Content);
    assert_eq!(tokens[1].tag_status, TagStatus::Undefined);

    assert_eq!(tokens[2].value, "");
    assert_eq!(tokens[2].token_type, TokenType::Script);
    assert_eq!(tokens[2].tag_status, TagStatus::Close);
}

#[test]
fn big_boy_test() {
    let tokens = tokenize("{#import component.flux}

{#script}
    console.log(context.value);
    function getPort() {
        return 80;
    }
{/script}

apiVersion: v1
kind: Service
metadata:
  name: my-service
spec:
  ports:
    {#each port in [0, 1, 2]}
    - protocol: TCP
      port: {#fn port}
      targetPort: {#fn getPort()}
    {/each}
   {#fn component(10)}");

    assert_eq!(tokens.len(), 16);
    assert_eq!(tokens[0].value, "component.flux");
    assert_eq!(tokens[0].token_type, TokenType::Import);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    assert_eq!(tokens[1].token_type, TokenType::Content);

    assert_eq!(tokens[2].token_type, TokenType::Script);
    assert_eq!(tokens[2].tag_status, TagStatus::Open);

    assert_eq!(tokens[3].token_type, TokenType::Content);

    assert_eq!(tokens[4].token_type, TokenType::Script);
    assert_eq!(tokens[4].tag_status, TagStatus::Close);
}