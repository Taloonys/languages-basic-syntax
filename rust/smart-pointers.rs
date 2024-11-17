/**
 * Smart pointers right from C++, that hold data in heap, but auto deallocate them when object holder is destroyed.
 * Borrow checker isn't smart enough to handle data structures cases, but smart ptrs is ok for safe memory management here.
 */


use std::rc::Rc;


fn main() {
    let _box = {                                                    // box implements single ownershiping concept
        let string_heap = Box::new(String::from("text"));           // alloc on heap
        println!("length is {}", string_heap.capacity());           // treat like to a value inside
        println!("{}", *string_heap);                               // that means a PASSED OWNERSHIPPING + MOVE TO STACK
    };

    let _rc = {                                     // rc - reference counter concept smart-ptr on IMMUTABLE DATA
        let i32_heap = Rc::new(555);
        let _i32_heap1 = i32_heap.clone();          // add 1 more owner, IT DOESN'T RLY COPY-CLONE DATA
    };                                              // `i32_heap` & `_i32_heap1` drop -> `555` is also dropped

    let _ref_cell = {                               // like `Rc` but with MUTABLE DATA, ofc it works in runtime
        let vec_heap = RefCell::new(vec![1,2]);

        let _vec_heap1 = vec_heap.borrow();         // got immutable ref

        let mut vec_heap2 = vec_heap.borrow_mut();  // got mutable ref
        vec_heap2.push(5);
    };

    let _cell = {                               // it's a signle Cell with IMMUTABLE DATA
        let i32_heap = Cell::new(555);
        let _i32_heap1 = i32_heap.clone();      // this operation COPY DATA in different memory space

        _i32_heap1.set(666);                    // this also cause COPY, rn there should be 3 different i32 "objects"
    };
}