fn main() {
    let age = 23;

    {
        println!("{}", age);

        let age = age;

        println!("{}", age + 10);
    }
    println!("{}", age);
}

/*
    Variable Freezing in Rust
    We can freeze a variable in Rust by using shadowing and immutability.
    Once a variable is frozen, we cannot change the variable value in the inner scope.
*/

fn main1() {
    let mut age = 1;

    // start of the inner block
    {
        // shadowing by immutable age variable
        let age = age;

        // error, age variable is frozen in this scope
        age = 2;

        println!("age variable inner block = {}", age);
        // age variable goes out of scope
    }
    // end of the inner block

    // age variable is not frozen in outer block
    age = 3;

    println!("integer variable outer block = {}", age);
}
