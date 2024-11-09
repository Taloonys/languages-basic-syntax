
#[derive(Debug)]
struct Data {                       // makes possible for `pretty printing` like `{:?}`
    title   : String,
    data    : i32
}


struct Data1 {
    data    : i32
}


fn input_keyboard() {
    let mut input_word = String::new();
    println!("Input any word");

    let _result = io::stdin().read_line(&mut input_word);    // read until new line
    println!("Got {input_word}");
}


fn main() {
    let title = "ClickHouse";
    let data_val = 999;

    println!("{title}, {data}");
    dbg!("{title}, {data}");                                // debug variant, also prints [folder/file:line:position]

    let data = Data {
        title: title.to_string(), 
        data: data_val
    };

    let data1 = Data1 {
        data : data_val
    };

    let _is_not_possible_without_derive_debug = {
        println!("{:#?}", data);
        println!("{:?}", data);

        // println!("{:?}", data1);                         /*<-- Err, not defined fmt::print or debug macro */
    };
}
