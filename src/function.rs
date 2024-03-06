// pass signle parameter
fn single_fn(c: i32) {
    println!("value of c is: {}", c);
}

// pass double parameers -> example of statements
fn double_fn(x: i32, y: char) {
    println!("value of x is: {} and value of y is {}", x, y);
}

// Simple function
fn first_fn() {
    println!("first function call");
}

// expressions
fn expression() {
    let y = {
        let x = 9;
        x + 1
    };

    println!("{y}");
}

// functoin return value
fn return_value(x: i32) -> i32 {
    return 78 + x;
}

// fn main() {
//     println!("HI tehre");

//     let a: i32 = 45;
//     println!("{}", a);

//     let name: &str = "user name";

//     println!("{name}");

//     first_fn();
//     single_fn(a);
//     double_fn(23, 'y');
//     expression();
//     let y = return_value(a);

//     println!("return value of y is: {}", y);
// }
