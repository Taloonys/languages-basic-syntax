struct Circle {                                                             /**< supportive example */
    radius: f32
}

 
//
// Default type param
//

trait Drawable<T = Circle> {                                        
    fn draw(&self, figure:&T);
}


struct CircleGui;                                                           /**< supportive example */


//
// Implementation for `T` as `Circle`, knwon as `conditional conformance`
//

impl Drawable<Circle> for CircleGui {           
    fn draw(&self, figure: &Circle) {           
        println!("A circle with radius {r}", r = figure.radius);                
    }
}


//
// No association with any Type looks like this
//

impl<T> Drawable<T> for Circle {                                    
    fn draw(&self, figure: &T) { println!("No-no, use gui") }
}


//
// Generic func with restriction in traits input
// It's call `Trait bound`
//

fn draw_figure<T>(drawable: &T, figure: &Circle) where T: Drawable {        // also `where` syntax
    drawable.draw(figure);                                                  // .. also ok for structs
}


//
// Use
//

fn main() {
    let gui     = CircleGui{};
    let circle  = Circle{radius: 2.0};

    gui.draw(&circle);
    draw_figure(&gui, &circle);
}