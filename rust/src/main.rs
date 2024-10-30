// Short comment 

/*
    Long comment
 */


fn main() {
    println!("Entry point");

    test();
}


fn test() {
    let block = {
        println!("init block");             // would be called rn
        let a = 5;
        999
    };

    block;                                  // ok
    // block();                             /*<-- err, not like this */
    println!("{block}");                    // will print return value -> `999`
}