use std::collections::{HashMap, HashSet};


fn main() {
    //
    // Vector
    //

    let _vector = {
        let mut v1: Vec<i32> = Vec::new();
        let mut v2 = vec![1, 2, 3];

        v1.push(555);

        println!("v2 first el is {first}", first = v2[0]);
        println!("v1 first el is {first:?}", first = v1.get(1));

        dbg!(v1.len());

        v2.remove(0);
    };

    //
    // String ~ Vec<u8>
    // + Slice
    //

    let _string_collection = {
        let text1 = String::new();              // String
        let _text2 = "Asd";                     // string literal*
        let text3 = "asd".to_string();          // String
        let text4 = String::from("asd");        // String

        dbg!("{}", text1.len());

        println!("{concatenation}", concatenation = text3 + &text4);

        let _slice_val = &text4[0..3];          // Slice
    };

    //
    // HashMap
    //

    let _hash_map = {
        let mut hmap1 : HashMap<i32, &str> = HashMap::new();
        let hmap2 = HashMap::from([
            ("tttt", 25),
            ("asd", 555)
        ]);

        let raw_data = vec![ ("Alice", 35), ("Tom", 39) ];
        let _people: HashMap<_, _> = raw_data.iter().cloned().collect();    // get iterator on data, clone stuff, collect in hmap

        hmap1.insert(999, "idk");
        dbg!("{} {}", hmap2["tttt"], hmap2.contains_key("asd"));
    };

    //
    // HashSet
    //

    let _hash_set = {
        let mut set1 = HashSet::from([
            1, 2, 3
        ]);

        let mut set2 = HashSet::from([
            2, 3, 4
        ]);

        let inter_set: HashSet<_> = set1.intersection(&set2).collect();
        println!("{:?}", inter_set);                                            // 2, 3 

        let diff_set: HashSet<_> = set1.difference(&set2).collect();
        println!("{:?}", diff_set);                                            // 1

        if inter_set.is_subset(&diff_set) { /* Empty */ }
    };
}