/**
 * `#[test]` attribute in rust usually search tests inside main, this attribute will trigger on `cargo test`
 * Ofc tests should be in a different module and there should be used `#[cfg(test)]`
 * 
 * I left only simple use case, because unit-testing is connected with cargo setup
 */


fn summ(a: i32, b: i32) -> i32 { a + b }


#[test]
fn test_summ() {
    let a = 10;
    let b = 50;

    let result = summ(a, b);

    assert_eq!(result, 60, "Err Summ");
}


fn main() { }