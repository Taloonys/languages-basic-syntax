fn main() {
    let mut a = 5;

    let const_pa: *const i32    = &a;               // just initing pointers is ok
    let pa: *mut i32            = &mut a;

    let const_pa2   = &a as *const i32;
    let pa2         = &mut a as *mut i32;

    unsafe {
        let imp_pa = *pa;                           // but unnaming pointers is unsafe
        println!("{imp_pa}");
    }

    let _unsafe_cell = {                            // unsafe version for Rust smart pointer `Cell`
        let i32_heap = UnsafeCell::new(555);

        unsafe {                                    // only unsafe operations could be done with data inside
            *i32_heap.get() = 100;
            println!("{}", *i32_heap.get());
        }    
    };
}