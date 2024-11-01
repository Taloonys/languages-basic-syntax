fn lambda_block() {
    let square = |x: i32| {
        println!("{}", x*x)
    };
    square(5);

    let better_square = |x: i32| -> i32 { x * x };
    better_square(5);
}


fn smart_lambda() {
    let smart_summ = |a, b| { a + b };                  // auto type deduction

    smart_summ(1,2);

    // smart_summ(1.2, 2.4);                               /*<-- error */

    /*
        compiler generates for that scope only 1 such function, 
        and its params are decided in first call 
    */
}

fn capture_block() {
    let mut x = 1;
    let mut increment = || {    // `mut` allow to change captured values
        x += 1;                 // captured from closest scope
    };
    
    increment();
    println!("x = {x}");
}
