//
// Could be used in funcs and structs
// Any single cases below could be intruded in each other "places" and "cases"
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


fn stuct_generics() {
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
        println!("{:b}", robots_greetings);                             // print in binary format:w
    }

    let pablo = Greetings::Spanish("Hola");
    if let Greetings::Spanish(spanish_greetings) = pablo {
        println!("{spanish_greetings}");
    }
}

//
// Expanded
//

struct Circle {
    radius: f32
}
 

trait Drawable<T = Circle> {                                        // default type param
    fn draw(&self, figure:&T);
}


struct CircleGui;


impl Drawable<Circle> for CircleGui {           // implementation for `T` as `Circle`
    fn draw(&self, figure: &Circle) {           // assume it to be Circle
        println!("A circle with radius {r}",    // we can use `radius` only due to `Circle` as `T`
            r = figure.radius);                 // .. so there are no attempts to instantiate as it should be...
    }
}


impl<T> Drawable<T> for Circle {                                    // standard
    fn draw(&self, figure: &T) { println!("No-no, use gui") }
}


// Trait bound
// [allow only some types for Generic]
fn draw_figure<T>(drawable: &T, figure: &Circle) where T: Drawable {        // also `where` syntax
    drawable.draw(figure);                                                  // .. also ok for structs
}

trait DummyTrait { 
    fn doNothing() { /* Empty */ } 
}

impl<T> DummyTrait for T                                                    // We could use generic such case
{ }                                                   

fn multiple_traits<T>(drawable: &(impl Drawable + DummyTrait))              // Multiple traits
{ }


//
// Use
//

fn trait_generics() {
    let gui     = CircleGui{};
    let circle  = Circle{radius: 2.0};

    gui.draw(&circle);
    draw_figure(&gui, &circle);
}