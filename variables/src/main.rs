fn main() {

    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    shadowing();
}

fn shadowing(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x); 
}