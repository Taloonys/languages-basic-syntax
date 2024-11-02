// Short comment 

/*
    Long comment
 */


fn main() {
    println!("Entry point");

    test();
}


fn test() {
    let person = ("John", 50);

    if let ("John", 50) = person {
        println!("Success case, corteges are equal");
    }

    if let ("John", age) = person {
        println!("case: {age} -> should be 50");
    }
}