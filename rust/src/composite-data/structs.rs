//
// all struct members should be used with any operation with structs
//

struct User {
    name        : String,
    age         : i32,
    is_java_usr : bool
}


fn struct_use() {
    let john = User{
        name        : "John".to_string(),
        age         : 55,
        is_java_usr : false
    };

    println!("{name} - {age} - enjoying java? -> {java_user}", 
        name        = john.name, 
        age         = john.age, 
        java_user   = john.is_java_usr);
}


fn struct_tricks() {
    let a_name      = "Keke";
    let age         = 22;
    let is_java_usr = true;

    let keke = User {                       // shorter 
        name : a_name.to_string(),          
        age,                                // if param name is only the same as struct member!!
        is_java_usr                         // .. same name
    };                       

    let keke_copy = User {
        name: "Cooler Keke".to_string(),
        ..keke                              // it will grab other members from `keke` obj
    };

    let _struct_decompose = {
        let User{
            name        : sd_name, 
            age         : sd_age, 
            is_java_usr : _                 // skip
        } = keke_copy;                      // decomposed keke_copy to params `sd_name` & `sd_age`

        println!("{sd_name}, {sd_age}");
    };
}