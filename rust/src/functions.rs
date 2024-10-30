fn summ(x: i32, y: i32) -> i32 { x + y }


fn form_card(name: &str, age: i32) {
    println!("-- name -> {}", name);
    println!("-- age  -> {}", age);
}


fn in_out_square(mut x: i32) { x = x^2; }   // didn't tested


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