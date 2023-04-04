use juxt::{compile_and_execute};
use juxt::engine::Juxt;

#[test]
fn test_simple() {
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: "Hello World".to_string(),
    }, Vec::new());
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
    }, Vec::new());
    assert_eq!(result, "\nport: 80");
}

#[test]
fn test_each() {
    let template = "{#each port in [0, 1, 2]}
      port: ${port}
    {/each}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, Vec::new());
    assert_eq!(result, "\n      port: 0\n      port: 1\n      port: 2");
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
    }, Vec::new());
    assert_eq!(result, "\n      port: 1\n      port: 2\n      port: 3");
}

#[test]
fn test_import() {
    let component = Juxt {
        name: "component".to_string(),
        template: "Hello world".to_string(),
    };

    let template = "{#import component.juxt}
    ${component()}";
    let result = compile_and_execute(Juxt {
        name: "main".to_string(),
        template: template.to_string(),
    }, vec!(component));
    assert_eq!(result, "\n    Hello world");
}