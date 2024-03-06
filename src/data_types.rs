fn main() {
    println!("data types is rust");

    // Scalar types - we can store single value
    // Int, strings, boolean, floating, char

    // length (8 bit, 16, 32,64, 128. arch) signed(i) - i16, or unsigned(u) - u32

    // let num: f32 = 34.5556;

    // let num_2: i32 = num as i32;

    // println!("{num_2}");

    // let alphabet: char = 'A';

    // let alp_num: i16 = alphabet as i16;

    // println!("{}", alp_num);

    // let val: u8 = 54;

    // let ch: char = val as char;

    // println!("{}", ch);
    // println!("Max u: {}", u8::MAX);

    // let val: bool = true;
    // let rest: i32 = 23;

    // let ans = val && rest != 0;

    // println!("{}", ans);

    let number: i32 = 2;
    println!("{number}");

    let is_male = true;

    println!("{is_male}");

    let char_ex = 'H';

    println!("{}", char_ex);

    // Compount type variables - multiple value
    // arrays, tuples, dictionary,

    // tuples
    let mut tup: (i32, i32, f32, i32) = (32, 34, 5.5, 6);

    tup.1 = 340;
    println!("{:?}", tup);
    println!("{}", tup.1);

    // Arrays

    let a: [i32; 5] = [1, 2, 3, 5, 6];

    println!("{:?}", a);

    println!("{}", a[3])
}
