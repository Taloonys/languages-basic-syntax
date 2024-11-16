//
// There are 2 types of errors: recoverable(any `monad` implementations) & unrecoverable ('panic')
//

fn panicking() {
    if 1 > 2 {
        panic!("Err");                       // <-- macro, that terminate the programm with error, runtime + compile time smt
    }

    let arr = [1,2];        
    // println!("{}", arr[4]);              /* <-- compile time error, because array is compile time context stuff */

    let v = vec![1, 2];                     // `!` implements panic
    // let _a = v[4];                       /* <-- runtime error, vector is dynamicly resizeable - runtime stuff */
}



fn resulting() -> Result<String, i32> {         // Result < Ok, Err >, imagine that enum or sth complex could be inside Result
    if 1 > 2 { 
        return Result::Ok(String::from("1 is more than 2"))
    }

    Result::Err(-1)
}


fn question_operator() ->  Result<String, i32> {
    let result_1_greater_2 = resulting()?;      // notice `?` operator
    Ok(result_1_greater_2)

    /*
    This is equal to: 
        match resulting() {
            Ok(result_1_greater_2)  => Ok(result_1_greater_2),
            Err(e)                  => Err(e),
        }
     */
}


fn main() {
    let _panicking = {
        panicking();
    };

    let _resulting = {
        let result_1_greater_2 = resulting();
        match &result_1_greater_2 {
            Ok(strr)    => println!("{strr}"),              // could use `unwrap` to extract from `Ok`, if Err -> panic
            Err(ec)     => println!("{ec}"),                // could `err` to extract from `Err`
        }

        let _t = result_1_greater_2.expect("got Err");      // like `unwrap`, but pass msg for panic
    };

    let _better_result_treating = {
        let _result_collect = question_operator();
    };
}