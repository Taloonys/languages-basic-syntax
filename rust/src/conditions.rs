//
// One of the key mechanics in rust is `pattern mathcing`
//

fn if_block() {
    if 5 != 0 
    { }
    else if 5 == 0
    { }
    else 
    { }
}


fn let_if_block() {
    let a = if false { 
        5 
    } else { 
        10 
    };
}


fn if_let_pattern() {
    let person = ("John", 50);

    if let ("John", 50) = person {
        println!("Success case, corteges are equal");
    }

    if let ("John", age) = person {
        println!("case: {age} -> should be 50");
    }
}



fn match_block() {
    let a = 10;
    match a {
        1 | 2 | 3   => println!("It's either 1, 2 or 3"),
        4           => println!("It's only 4"),
        5..7        => println!("It's in range [5..7)"),
        7..=9       => println!("It's in range [7..9]"),
        _           => println!("Default scope")
    }


}


//
// also works with any struct, array, enum, cortege and e.t.c
//
struct Person{
    age     : i32
}

fn pattern_match() {
    let pos = ("Me", 123123);
    match pos {
        (_, 3)      => println!("at least num is right"),
        ("Me", _)   => println!("?"),
        _           => ()
    }

    let john = Person{
        age     : 55
    };

    match john {    
        Person {age: his_age @ 35..61}      => println!("Hi is just {his_age}"),    // in-line param
        Person {age} if age > 0 && age < 10 => println!("He is young"),             // condition pattern
        _                                   => println!("_")
    }
}