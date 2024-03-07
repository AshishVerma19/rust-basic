use std::collections::HashMap;

fn main() {
    // HashMap
    let mut my_map: HashMap<String, i32> = HashMap::new();

    let key = String::from("Ashish");
    my_map.insert(key, 23);

    println!("{:?}", my_map.get(&String::from("Ashish")).unwrap());

    for v in my_map.keys() {
        println!("{}", v);
    }
}
