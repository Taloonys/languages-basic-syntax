struct Circle {
    radius: f32
}

struct Rectangle {
    a: i32,
    b: i32
}

trait Shape {
    type RetType;

    fn calc_square(&self) -> Self::RetType;             // `Self` is a keyword for struct type that would get that trait impl
}

impl Shape for Circle { 
    type RetType = f32;

    fn calc_square(&self) -> Self::RetType {
        (std::f64::consts::PI as f32) * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    type RetType = i32;

    fn calc_square(&self) -> Self::RetType {
        self.a * self.b
    }
}


//
// Example for dynamic traits
//

trait DummyTrait {
    fn idk(&self);
}

impl DummyTrait for Circle {
    fn idk(&self) { }
}

//
// Rust decide at compile time, which traits object has, but we can dynamicly dispatch which trait we a gonna stick to
// by using ...`&dyn` = &...
//

fn main() { 
    let circle : &dyn DummyTrait = &Circle { radius: 5.0 };         // we borrow a trait stuff from Circle in run time
    let rect   = Rectangle { a: 5, b: 5 };

    circle.idk();
    // circle.calc_square();                                        /*<-- Err, we inly picked DummyTrait */

    rect.calc_square();
}


/*
    //
    // It also allows to do concept `type-level programming`
    //

    fn attack<C: Character>() {
        let weapon = C::create_weapon();    // aka named constructor as trait method, works due to `Self`
        weapon.attack();                    // working with trait's properties
    }

 */