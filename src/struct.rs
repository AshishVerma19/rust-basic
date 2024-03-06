struct Pearson {
    name: String,
    age: u8,
}

fn main() {
    let ashish: Pearson = Pearson {
        age: 23,
        name: String::from("Ashish Verma"),
    };

    println!("name is: {} and age is {}", ashish.name, ashish.age);

    // destructure struct
    // NOTE: if we don't want to destructre rest of the fields, use .. at the end
    let Pearson { name, .. } = ashish;

    println!("{}", name);

    let name = "    ";
    let name_len = name.len();
    println!("{}", name);
    println!("{}", name_len);
}
