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


fn match_block() {
    let a = true;
    
    match a
    {
        true    => println!("ok"),
        false   => println!("not ok"),
        _       => println!("default")
    }
}