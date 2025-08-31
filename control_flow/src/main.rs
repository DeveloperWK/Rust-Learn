fn main() {
   let num = 3;
    if num<5{
        println!("num is 5");
    }else {
        println!("condition was false");
    }
    let number = 3;
     // if number {
     //   println!("number was three"); Its Error
    //}
    if number != 0 {
        println!("number was something other than zero");
    }
    let number_1 = 6;
    if number_1 % 4 ==0{
        println!("number is divisible by 4");
    }else if number_1 % 3 ==0 {
        println!("number is divisible by 3");
    }else if number_1 % 2 ==0 {
        println!("number is divisible by 2");
    }else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let num_1 = if condition {6} else {    5 }; // Here must match if and else arm types
    println!("The value of num_1 is {}", num_1);
}
