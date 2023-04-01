use js_sandbox::{Script, AnyError};


fn eval() {
    let js_code = "function triple(a) { return 3 * a; }";
    let script = Script::from_string(js_code).unwrap();

    let arg = 7;
    let result: i32 = script.call("triple", &arg).unwrap();

    assert_eq!(result, 21);
    println!("{} * 3 = {}", arg, result);
    // Ok(())
}