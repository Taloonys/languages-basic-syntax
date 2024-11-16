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
}