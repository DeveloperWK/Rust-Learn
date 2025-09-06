fn main() {
    // {
    // let s = "hello";
    //    println!("{}",s)
    // }
    // let s = String::from("hello")
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{s}");
    // let x = 5;
    // let y = x;
    // println!("{y}");
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{s1}").

    // let mut s = String::from("Hello");
    // s = String::from("Wasi");
    // println!("{s}, World")

    // let s = String::from("Hello");
    // takes_ownership(s);

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("{s3}")

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}
fn takes_ownership(s: String) {
    println!("{s}")
}
fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
