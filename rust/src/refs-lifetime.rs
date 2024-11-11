/**
 * Sometimes rust requires specifying reference's lifetime. It doesn't expand it. 
 * Defying ref's lifetimes just define connection between this lifetimes
 */


fn get_ref_string<'a>() -> &'a str {                    // when we return ref, we have to specify its lifetime
    "string"                                            // return string literal has the same lifetime mark ` 'a `
}


fn multi_refs<'a, 'b>(x: &'a str, y: &'b str) { }       // 'a & 'b are just different lifetimes, nothing more


struct Person <'a> {
    name    : &'a str,                                  // we have string-literal ref, so we have to specify lifetime
    idea    : &'static str                              // this ref would be alive the whole time (as any static value)
}


impl<'a> Person<'a> {
    fn print_name(&self){
        println!("{name}", name = self.name);
    }
}


fn main() {
   println!("{str}", str = get_ref_string());
}