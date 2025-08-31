use std::io;
fn main() {
    let a = [1,2,3,4,5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
// let month1 = months[0];
//     println!("month1 {:?}", month1);
    println!("Please enter an integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index:usize = input.trim().parse().expect("Please type a number!");
    let el = a[index];
    println!("The value of the element at index {index} is: {el}");


}
