fn my_function(y: i32) {
    println!("The value of y is: {}", y);
}

fn main() {
    my_function(999);
    let x = plus_one(5);   
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
