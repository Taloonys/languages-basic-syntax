fn tuple_block() {
    let user_data = 
        ("Gigachad", 50, 2.22, "Vim enjoyer");

    let user_data1: (&str, i32, f32, &str) = 
        ("Gigachad", 50, 2.22, "Vim enjoyer");

    println!("{who}, {age}", 
        who = user_data.0, 
        age = user_data.1);

    let tuple_decompose = {
        let (user, age, height, title) = user_data1;        // extract all tuple members in one line
        // let (user, age) = user_data1;                    /*<-- err, should use all */
    };

    let tuple_input = |(pos_name, pos_age):(&str, i32)| {   // ofc, types could be skipped
        println!("{pos_name}, {pos_age}") 
    };
}


fn array_block() {
    let numerals: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];       // full describe
    println!("{:?}", numerals);                                     // pretty printing
    println!("{five}", five = numerals[5]);
    println!("{len}", len = numerals.len());

    let _default_fill = {
        let default_value   = 1;
        let five_ones = [default_value; 5];

        println!("{:?}", five_ones);
    };

    let _some_operations = {
        let mut numerals_copy: [i32; 10] = numerals;               // copy, i32 support Copy trait
        numerals_copy.sort();

        let _num_full_copy: [i32; 10] = numerals.to_owned();       // if doesn't support Copy trait
    };
}


struct Position (
    f32,            // x
    f32             // y
);

fn cortege_struct() {
    let my_pos = Position(33.4, 55.5);
    println!("I'm at pos {x}, {y}",
        x = my_pos.0,
        y = my_pos.1
    );

}