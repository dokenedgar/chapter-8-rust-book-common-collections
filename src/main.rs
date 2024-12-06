mod number_one;
mod number_two;
fn main() {
    // let mut num_one_arr = [5, 7, 9, 23, 1, 2, 3, 4, 9];
    let result = number_one::median_mode(&mut [5, 7, 9, 23, 1, 2, 3, 4, 9]);
    println!("{:?}", result);
    let number_two_result = number_two::pig_latin("mean");
    println!("{number_two_result}")
}
