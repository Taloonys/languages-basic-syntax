fn structs_with_no_copy() {
    let user  = (String::from("Tom"), 39);      // includes obj with no CopyTrait impl
    let employee = user;                        // that applyies `move` instead of `copy``
 
    // println!("User Name: {}", user.0);       /*<-- Err, user was moved*/
    println!("Employee Name: {}", employee.0);
}