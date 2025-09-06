fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(s: &mut String) {
    s.push_str(", bangladesh");
}
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    // let r2 = &mut s; it's Error Because One Mut ref at a time
    let r2 = &mut s; // Now it's work fine for r1 scope

    let r3 = &s; // no problem
    let r4 = &s; // no problem
    let r5 = &s; // no problem
    println!("{r3} and {r4},{r5}");
    change(&mut s);
    println!("{s}")
}
