/*
 * Btw, there is not void-like type, if function doesn't return anything, it's `->` is ommited
 */
fn summ(x: i32, y: i32) -> i32 { x + y }


fn form_card(name: &str, age: i32) {
    println!("-- name -> {}", name);
    println!("-- age  -> {}", age);
}


fn mut_input(mut x: i32) {                  // `mut` word
    x = x * 10;                             // for changing `x` inside function scope
}


fn code_block() {
    let block = {
        println!("init block");             // would be called rn
        let a = 5;
        999
    };

    block;                                  // ok
    // block();                             /*<-- err, not like this */
    println!("{block}");                    // will print return value -> `999`
}


fn func_as_value() {
    let f_summ: fn(i32, i32) -> i32 = summ;
    println!("f_summ = {}", f_summ(3, 3));
}


fn func_as_input(op: fn(i32, i32) -> i32) {        // same for return case...
    println!("func_input_summ = {}", op(3, 3));    // assume to pass summ func, for example
}
