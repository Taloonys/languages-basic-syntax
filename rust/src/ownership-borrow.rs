fn structs_with_no_copy() {
    let user  = (String::from("Tom"), 39);      // includes obj with no CopyTrait impl
    let employee = user;                        // that applyies `move` instead of `copy``
 
    // println!("User Name: {}", user.0);       /*<-- Err, user was moved*/
    println!("Employee Name: {}", employee.0);
}


fn drop_info() {
    let tmp: String = "Text".to_string();   // `String` param locates it's data inside heap
    println!(tmp);
    drop(tmp);                              // manual `drop`
}   /*<-- drop usually is called here for every single param declared in this scope */


fn borrowing_info() { 
    //
    // Rust implement reference checker mechanism called `borrow checker`, it works at compile time
    // borrow checker rules -> https://rustc-dev-guide.rust-lang.org/borrow_check.html
    // one of the most important rules -> it's IMPOSSIBLE TO HAVE MORE THAN 1 MUTTABLE REF SIMULTANEOUSLY
    //

    let s1 = "Text".to_string();
    let s2 = &s2;                   // `s2` is `&String`

    let a = 5;
    let ref_a = &a;
    // let b = ref_a < a;           /*<-- Err, `ref` < `i32` */
    let b = *ref_a < a;             // We have to deref it (like pointers usually)
}


fn mutable_ref() {
    let add_world = |w: &mut String| { w.push(" World") };      // lambda with mutable ref for param
    let mut hello = "Hello".to_string();
    add_world(&mut hello);                                      // declare that we pass as mutable ref
}


fn ownership_in_closures() {
    let a = "ttt".to_string();                      // String doesn't have Copy Trait, so it can be only moved

    let better_lam = || { 
        let tmp = &a;                               // copied ref to value 
        println!("{tmp}");
    };          

    better_lam();                                   // can calll multiple times
    better_lam();

    let lam = || { 
        let tmp = a;                                // basicly we `move` ownership instead of `copy`
        println!("{tmp}");
    };  

    lam();
    // lam();                                       /*<-- err, cuz tmp was moved at first `lam` call */

    // better_lam();                                /*<-- we can't borrow `a`, cuz it was already moved by `lam` call */
}