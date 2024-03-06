fn main() {
    // Closure

    // let print_text = |text: String| println!("sample data printing: {}", text);

    // print_text(String::from("Sample value"));

    let add_numbers = |a: i32, b: i32| {
        let sum = a + b;
        return sum;
    };

    let sum = add_numbers(1, 23);

    println!("{}", sum);

    let word = String::from("Hello");

    // immutable closure
    let print_str = || {
        // word variable is moved to a new variable
        let new_word = word;
        println!("word = {}", new_word);
    };

    print_str();

    // cannot immutable borrow because word variable has moved inside closure
    // println!("length of word = {}", word.len());

    // let mut word = String::from("Hello");

    // // mutable closure
    // let mut print_str = || {
    //     // value of word is changed here
    //     word.push_str(" World!");
    //     println!("word = {}", word);
    // };

    // // cannot immutable borrow because the variable is borrowed as mutable inside the closure
    // // println!("length of word = {}", word.len());

    // print_str();
    // print_str();
    // print_str();

    // // can immutable borrow because the closure has been already used
    // println!("length of word = {}", word.len());
}
