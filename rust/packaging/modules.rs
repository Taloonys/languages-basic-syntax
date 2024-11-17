//
// It's like a controllable namespaces in C++, there could be anything inside (struct, enums & etc)
//


mod access_module {
    pub fn test() {                                 // makes test `public visible` from outside
        println!("Invoked test");
        test_();                                    // private func could be used
    }      

    fn test_() { }                                  // only visible inside module
}


mod nesting_module {
    fn test_() { }

    pub mod inner_module {                          // nested module
        pub fn test() {                             // public
            println!("Invoked inner test");
            super::test_();                         // use outer module scope
        }

        pub fn test2() { }
    }
}


use access_module::*;                               // using whole module in symbol lookup, so we don't need to specify mod name
use nesting_module::inner_module::{test, test2};    // mutiple & concrete
use access_module::test as access_test;             // use alias


fn main() {
    test();                                         // also `inner_module` context, so `use` should be written FIRST
    use nesting_module::inner_module::test;         // shadowed previous using mod in current scope for `test()`
    test();                                         // `inner_module` context

    access_module::test();
    nesting_module::inner_module::test();

    access_test();                                  // using use alias
}