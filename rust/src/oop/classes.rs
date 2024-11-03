//
// Rust is a bit different in terms of OOP, there are objects with their properties & traits for them.
// Traits could be sticked "additionally" to any abstraction
//


struct Vehicle {
    wheels : i32,
    vtype  : String
}

impl Vehicle {
    fn start_driving (wheels: i32, vtype: String) -> Vehicle {      // associative method ~ aka `static` method
        Vehicle { wheels, vtype }                                   // pattern matching
    }

    fn info(&self) {                                                // struct method -> pass `self` by ref
        println!("type is {my_type}, wheel number = {nwheel}",
                    my_type = &self.vtype,                          // read values
                    nwheel  = &self.wheels)
    }

    fn info1(self) {                                                // no ref -> means -> pass ownershipping
        println!("type is {my_type}, wheel number = {nwheel}",
                    my_type = self.vtype,
                    nwheel  = self.wheels)
    }

    fn demontage(&mut self, wheels: i32) {
        self.wheels -= wheels;
        println!("now vehicle {t} with {w} wheels",
                    t = &self.vtype,
                    w = &self.wheels)
    }
}

 
fn usage() {
    //
    // aka named constructor
    //
    let mut car = Vehicle::start_driving ( 
        4,
        "Bicycle".to_string()
    );

    car.info();

    car.demontage(2);

    car.info1();                                // use movable stuff in the end for example
    // car.wheels;                              /* <-- Err, cuz we passed `wheels` by calling `info1` */
 }
