use juxt::{compile_and_execute};
use juxt::engine::Juxt;

#[test]
fn test_simple() {
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: "Hello World".to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "Hello World");
}

#[test]
fn test_function() {
    let template = "{#script}
    function getPort() {
        return 80;
    }
{/script}
port: ${getPort()}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "port: 80");
}

#[test]
fn test_each() {
    let template = "{#each port in [0, 1, 2]}
      port: ${port}
    {/each}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "      port: 0\n          port: 1\n          port: 2\n    ");
}

#[test]
fn test_function_each() {
    let template = "{#script}
    function plusOne(n) {
        return n + 1;
    }
{/script}
    {#each port in [0, 1, 2]}
      port: ${plusOne(port)}
    {/each}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "          port: 1\n          port: 2\n          port: 3\n    ");
}

#[test]
fn test_import() {
    let component = Juxt {
        name: "component.juxt".to_string(),
        template: "Hello world".to_string(),
    };

    let template = "{#import component.juxt}
    ${component()}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, vec!(component), String::new()).unwrap();
    assert_eq!(result, "    Hello world");
}

#[test]
fn test_import_script() {
    let component = Juxt {
        name: "script.js".to_string(),
        template: "function getPort() {
        return 80;
    }".to_string(),
    };

    let template = "{#import script.js}
    ${getPort()}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, vec!(component), String::new()).unwrap();
    assert_eq!(result, "    80");
}

#[test]
fn test_if() {
    let template = "{#if 1 == 1}\
    asd\
    {/if}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "asd");

    let template = "{#if 1 != 1}\
    asd\
    {/if}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_if_else() {
    let template = "{#if 1 === 1}\
    asd\
    {/if}\
    {#else}\
    dsa\
    {/else}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "asd");

    let template = "{#if 1 !== 1}\
    asd\
    {/if}\
    {#else}\
    dsa\
    {/else}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::new()).unwrap();
    assert_eq!(result, "dsa");
}

#[test]
fn test_context() {
    let template = "${context.test}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new(), String::from("{\"test\": \"hi\"}")).unwrap();
    assert_eq!(result, "hi");
}