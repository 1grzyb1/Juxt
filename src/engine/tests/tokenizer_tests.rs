use crate::engine::tokenizer::{TagStatus, tokenize, TokenType};

#[test]
fn should_tokenize_content() {
    let tokens = tokenize("some test").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "some test");
    assert_eq!(tokens[0].token_type, TokenType::Content);
    assert_eq!(tokens[0].tag_status, TagStatus::Undefined);
}

#[test]
fn should_tokenize_import() {
    let tokens = tokenize("{#import component.flux}").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "component.flux");
    assert_eq!(tokens[0].token_type, TokenType::Import);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);
}

#[test]
fn should_tokenize_import_and_content() {
    let tokens = tokenize("{#import component.flux} blbablb").unwrap();
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
    let tokens = tokenize("{#script}").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[0].token_type, TokenType::Script);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    let tokens = tokenize("{# script  }").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].value, "");
    assert_eq!(tokens[0].token_type, TokenType::Script);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);
}

#[test]
fn should_tokenize_open_cloased_and_content() {
    let tokens = tokenize("{#script} bigos bigos {/script}").unwrap();
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
fn should_tokenize_if() {
    let tokens = tokenize("{#if 1 == 1} bigos bigos {/if}").unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, "1 == 1");
    assert_eq!(tokens[0].token_type, TokenType::If);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    assert_eq!(tokens[1].value, " bigos bigos ");
    assert_eq!(tokens[1].token_type, TokenType::Content);
    assert_eq!(tokens[1].tag_status, TagStatus::Undefined);

    assert_eq!(tokens[2].value, "");
    assert_eq!(tokens[2].token_type, TokenType::If);
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
      port: {$port}
      targetPort: {$getPort()}
    {/each}
   {$component(10)}").unwrap();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].value, "component.flux");
    assert_eq!(tokens[0].token_type, TokenType::Import);
    assert_eq!(tokens[0].tag_status, TagStatus::Open);

    assert_eq!(tokens[1].token_type, TokenType::Content);

    assert_eq!(tokens[2].token_type, TokenType::Script);
    assert_eq!(tokens[2].tag_status, TagStatus::Open);

    assert_eq!(tokens[3].token_type, TokenType::Content);

    assert_eq!(tokens[4].token_type, TokenType::Script);
    assert_eq!(tokens[4].tag_status, TagStatus::Close);

    assert_eq!(tokens[5].token_type, TokenType::Content);

    assert_eq!(tokens[6].token_type, TokenType::Each);
    assert_eq!(tokens[6].tag_status, TagStatus::Open);

    assert_eq!(tokens[7].token_type, TokenType::Content);

    assert_eq!(tokens[8].token_type, TokenType::Each);
    assert_eq!(tokens[8].tag_status, TagStatus::Close);

    assert_eq!(tokens[9].token_type, TokenType::Content);
}