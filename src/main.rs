fn main() {
    let number = 22;
    fibonacci_number(number);
}

fn fibonacci_number(x: i32) {
    let mut i = 1;
    let mut h: i32 = 0;
    let mut v: i32 = 1;
    let mut u: i32; 
    while h < x {
        u = v;
        v = i;
        i = i + u;
        h += 1;
        println!("{}: {}", h, i);
    }
}
