mod number_one;
fn main() {
    let mut num_one_arr = [5, 7, 9, 23, 1, 2, 3, 4, 9];
    let result = number_one::num_one(&mut num_one_arr);
    println!("{:?}", result)
}
