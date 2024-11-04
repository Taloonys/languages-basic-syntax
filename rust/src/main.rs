// Short comment 

/*

 */


fn main() {
    println!("Entry point");

    test();
}

struct Circle {
    radius: f32
}
 
trait Drawable<T = Circle> {                                        // default type param
    fn draw(&self, figure:&T);
}

struct CircleGui;

impl Drawable<Circle> for CircleGui {                               // implementation for `T` as `Circle`
    fn draw(&self, figure: &Circle) {                               // assume it to be Circle
        println!("A circle with radius {r}",                        // we can use `radius` only due to `Circle` as `T`
            r = figure.radius);                                     // .. so there are no attempts to instantiate as it should be...
    }
}

impl<T> Drawable<T> for Circle {                                    // standard
    fn draw(&self, figure: &T) { println!("No-no, use gui") }
}


fn test() {
    let gui = CircleGui{};
    let circle = Circle { radius: 2.0 };

    gui.draw(&circle);
}