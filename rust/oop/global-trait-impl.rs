//
// Trait implementation for "any" type
//

trait Greeter {
    fn hello(&self); 
}


impl<T> Greeter for T {                             // implement for any type
    fn hello(&self) { println!("Hello World"); }
}

trait Greeter2 {
    fn hello2(&self);
}

impl<T: Greeter> Greeter2 for T {                   // implement for any type with Greeter trait
    fn hello2(&self) { println!("Hola"); }
}

fn main() {
    let val = 5.55;
    val.hello();
    val.hello2();
}