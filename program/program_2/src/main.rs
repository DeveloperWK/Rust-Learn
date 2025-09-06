
fn fibonacci(n:u32)->u64{
    if n<=1{
        return n as u64;
    }
    let mut a:u64 = 0;
    let mut b:u64 = 1;
    for _ in 2..n+1{
        let temp = a+b;
        a=b;
        b=temp;
    }
    b
}

fn main() {
    let n = 10;
    println!("Fibonacci({}) = {}", n, fibonacci(n));
}
