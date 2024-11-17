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


fn closure_types() {
    /**
     * 3 types of lambda:
     *  - Fn        - only read values
     *  - FnMut     - can change values
     *  - FnOnce    - takes ownership
     * Usually it's auto deducted
     */

    let mut a = 5;
    let fn_lambda = || {                                    // Fn
        println!("a={a}, could only be `read`");
    };

    fn_lambda();
    println!("{a}");

    let mut fn_mut_lambda = || {                            // FnMut
        a += 10;
        println!("a in fn_mut_lambda = {a}");
    };

    fn_mut_lambda();
    println!("{a}");

    let t = 22;                                             // declare immutable value
    let fn_once_lambda = || {                               // FnOnce
        let mut b = t;                                      // get ownership
        println!("in fnOnce lambda 22 turned into {new_b}", 
                    new_b = b + 10);
    };

    fn_once_lambda();

    // t += 10;                                             /*<-- Err, we already "moved" `t` */

    let d = 22;
    let move_lambda = move || {                             // keyword `move`, that makes closure to pass ownership
        println!("{d}");                                    // .. but lambda type is still `Fn` or `FnMut`
    };

    // d += 10;                                             /*<-- Err, we moved `d` */
}