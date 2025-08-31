fn main() {

    let tup = (100, 7.4, 1);
    let (a,b,c)=tup;
    println!("Hello, world! {},{},{}",tup.0,tup.1,tup.2);
    println!("Tuple {},{},{}",a,b,c);
    let tuple:(i32,f32,u8)=(-32,4.5,17);
    println!("Tuple Type {},{},{}",tuple.0,tuple.1,tuple.2);
}
