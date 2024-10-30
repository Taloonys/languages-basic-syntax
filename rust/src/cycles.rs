fn loop_block() {
    let mut a = 0;

    loop                    // aka while true
    {
        println!("a is {}", a);
        if a == 2 {
            break;          // `break`
        }

        a += 1;
    }

    let b = loop {          // ofc can use for variable
        if a == 5 {
            break a * 10;   // return `break`
        }

        a += 1;
    };
    println!("b is {}", b);
}


fn while_block() {
    let mut a = 0;

    while a < 5 {
        a += 1;
    }
}


fn for_block() {
    for num in 1..5 
    {
        println!("{} ", num);
    }
}


fn cycle_marks() {
    'outer: loop {                  // mark loop
        let mut a = 0;

        loop {
            if a == 2 {
                break 'outer;       // break with mark
            }

            a += 1;
        }
    }
}