use std::any::type_name;


fn type_of<T>(_:T)->&'static str {
    type_name::<T>()
}

fn main() {
    // Mut Stand for Mutable
    // let mut x = 12;
    // x =23;
    let y:i8 = 127;
    println!("{}",y);
    let d = 255;
    let c = y.wrapping_add(10);
    println!("c:{}",c);
    println!("{}",type_of(d));

}
