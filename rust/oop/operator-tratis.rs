//
// Rust operators are implemented by traits
//

use std::ops::Add;
 
struct Counter { 
    value: u32
}
 
impl Add for Counter {
    type Output = Counter;

    fn add(self, other: Counter) -> Counter { 
        Counter { value: self.value + other.value }
    }
}


fn main() {
    let c1 = Counter{value: 5};
    let c2 = Counter{value: 10};

    let c3 = c1 + c2;                       // this is basicly `Add` Trait
}