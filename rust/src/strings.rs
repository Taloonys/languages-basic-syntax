fn formatting() {
    let name = "John";
    let age  = 55;

    println!("Name is {name}, age is {age}");

    /*
     * sad story: we can't use just this
     * println!(name, age);
     */
}


fn char_symbol() {
    let b: char = 'b';           // it's in unicode by default and takes 4 bytes
}


fn string_init() {
    let text: &str = "text";    // that's a string, array of UTF-8 symbols
}


fn shadowing() {
    let a: i32  = 32;
    let a: &str = "string";     // it's ok

    let mut a: i32 = 32;
    //a = "string"              /*<-- error */
}