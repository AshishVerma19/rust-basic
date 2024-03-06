fn main() {
    // if else

    // let val = -1;

    // if val > 5 {
    //     println!("value is greater than 5");
    // } else if val > 0 {
    //     println!("value is greater than 0 but smaller than 5");
    // } else {
    //     println!("value smaller than 0");
    // }

    /*
       looping
       1. loop
       2. while
       3. for
    */

    // let mut count = 0;

    // loop {
    //     println!("{}", count);
    //     count += 1;

    //     if count > 10 {
    //         break;
    //     }
    // }

    // while loop

    // let mut count: i32 = 0;

    // while count <= 10 {
    //     println!("{}", count);
    //     count += 1;
    // }

    // for in loop -> runs on range
    /*
        for variable in lower_bound_number..upper_bound_number {
        // code block
    }
     */

    // for i in 1..10 {
    //     println!("{}", i);
    // }

    // for loop with array

    // let arr: [i32; 5] = [1, 2, 3, 5, 6];

    // for val in arr {
    //     if val == 3 {
    //         break;
    //     }
    //     println!("{}", val);
    // }

    // Arrays

    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let arr1: [i32; 5] = [10; 5];

    // println!("simple arr: {:?}", arr);
    // println!("seconde arr: {:?}", arr1);

    // for val in 0..5 {
    //     println!("{} ", arr[val]);
    // }

    // slice works with collection
    /*
       &variable[start_index..end_index];

       start_index and end_index both can be avoided

       &arr[0..];
       &arr[..5];
       &arr[..];
    */

    // range 0 in inclusive 3 in not
    // let arr_slice: &[i32] = &arr[0..3];

    // for i in arr_slice {
    //     println!("{}", i);
    // }

    // Mutable slice

    // let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let slice_arr = &mut arr[..];

    // slice_arr[2] = 100;

    // for i in slice_arr {
    //     println!("{i}");
    // }

    // tuple

    // let mixer: (i32, i32, &str, char) = (1, 2, "ashish", 'A');
    // let mut mixer: (i32, i32, &str, char) = (1, 2, "ashish", 'A');

    // mixer.0 = 200;

    // println!("{:?}", mixer);
}
