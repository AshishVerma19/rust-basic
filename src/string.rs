fn main() {
    // String

    let mut name = String::from("Sample string");
    // Empty string
    let address = String::new();
    let temp = &name[3..];

    println!("{}", temp);

    name.push_str(" demo word");
    println!("{}", name);

    // for s in name.chars() {
    //     println!("{}", s);
    // }
}
