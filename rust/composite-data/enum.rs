enum Week {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

fn simple_enum() {
    let day = Week::Tuesday;

    match bad_day {
        Week::Friday..Week::Sunday  => println!("Goodday"),
        _                           => println!("Halp")    
    }
}


enum DescBool {                                                         
    True(String),                                                       // stick with storage types 
    False(String, String)
}

fn stick_type_enum() {
    let true_val    = DescBool::True("100% sure".to_string());
    let false_val   = DescBool::False("I didn't".to_string(), 
                                      "But tomorrow...".to_string());

    have_you_done_that_job(&true_val);
    have_you_done_that_job(&false_val);
}

fn have_you_done_that_job(ans: &DescBool) {                             // dead imagination rn
    let question = "Have you done your job?";

    //
    // Pick enum storaged values
    //
    match ans {
        DescBool::True(st)          => println!("{question} - {st}"),   
        DescBool::False(st, jstf)   => println!("{question} - {st}... {jstf}")
    }
}