//
// While in C++ and other languages it's just like a substitution attempts
// Generics in rust is a built-in feature, that could be checked in compile time
//

struct Stormtrooper <Id, Experience> {
    id      :   Id,
    weapon  :   String,
    exp     :   Experience
}


enum Greetings<T> {
    Binary(T),
    Spanish(T)
}


fn main() {
    let _sniper = Stormtrooper{
        id      : 5, 
        weapon  : "RXA-101".to_string(),
        exp     : 50
    };

    let _driver = Stormtrooper{
        id      : "689",
        weapon  : "D-11".to_string(),
        exp     : 16.6
    };

    let robo_hello = Greetings::Binary(0b0100100001101001);             // it's "Hi" =)
    if let Greetings::Binary(robots_greetings) = robo_hello {
        println!("{:b}", robots_greetings);                             // print in binary format
    }

    let pablo = Greetings::Spanish("Hola");
    if let Greetings::Spanish(spanish_greetings) = pablo {
        println!("{spanish_greetings}");
    }
}


/**
 * Any operator in rust is defined by it's own Trait, such as: Div, DivAssign, Add, AddAssign, Drop, Neg, Not & etc
 */