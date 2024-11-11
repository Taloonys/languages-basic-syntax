fn formatting() {
    let name = "John";
    let age  = 55;

    println!("Name is {name}, age is {age}");           // string interpolation
    println!("Name is {}, age is {}", name, age);       // as ordered
    println!("Name is {1} , age is {0}", age, name);    // set positions
    // println!("Name is {1} , age is {0}", age);       /*<-- err, not argument for {1} */
    println!("{name}, {age}, {end_title}",              // add new "argument"
             end_title="..end..");                      // .. specify new argument
    println!("{num:>5}", num=5);                        // 5 whitespaces before `num`
    println!("{num:0>5}", num=5);                       // 5 `0` before `num`
    println!("{num:0<5}", num=5);                       // 5 `0` after `num`
    println!("{:?}", age);                              // pretty printing, look for fmt

    dbg!("{name}, {age}");                              // debug variant, also prints [folder/file:line:position]
                                                        
    /*
     * sad story: we can't use just this
     * println!(name, age);
     */
}


fn char_symbol() {
    let b: char = 'b';           // it's in unicode by default and takes 4 bytes
}


fn string_init() {
    let text: &str = "text";            // that's a `string literal`, array of UTF-8 symbols, they live as `static` in memory
                                        // .. &str is like a view to immutable array of chars
    let text2 = String::from("text");   // that's pointer + size
}


fn shadowing() {
    let a: i32  = 32;
    let a: &str = "string";     // it's ok

    let mut a: i32 = 32;
    //a = "string"              /*<-- error */
}
