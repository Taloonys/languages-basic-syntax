//
// There is no explicit inheritance in Rust, it's a garbage code design mechanic, so ..
// Rust shadowed it by Traits system.. and macro for working with them sometimes
//

struct Square {
    a   : i32,
}


struct Rectangle {
    a   : i32,
    b   : i32,
}


enum EvalParallelogram {                                    // like evaluatable parallelogram
    Square(Square),
    Rect(Rectangle),
}


trait Area {                                                // trait that allows to calc area
    fn area(&self) -> i32 ;
}


impl Area for EvalParallelogram {                           // stick trait to group of structs
    fn area(&self) -> i32 {
        match self {
            EvalParallelogram::Square(s)    => s.a * s.a,   // a^2
            EvalParallelogram::Rect(r)      => r.a * r.b,   // a * b
        }
    }
}


fn print_area(obj: &impl Area) {                            // stick to trait
    println!("object area is {}", obj.area());
}


fn main() {
    let square  = EvalParallelogram::Square(Square {
        a: 5
    });

    let rect    = EvalParallelogram::Rect(Rectangle {
        a: 5,
        b: 2
    });

    print_area(&square);
    print_area(&rect);
}