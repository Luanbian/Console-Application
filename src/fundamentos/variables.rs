pub fn imutables() {
    let x = 5;
    // x = 10;
    println!("x => {}", x);
}

pub fn mutables() {
    let mut x = 10;
    let y = x;
    println!("x,y => {},{}", x, y);

    x = 20;
    println!("x,y => {},{}", x, y);
}

pub fn constants() {
    const X: u32 = 10;
    println!("X => {}", X);
}

pub fn shadowing() {
    let a = 32;
    println!("O valor é {}, do tipo i32", a);

    let a = String::from("Text");
    println!("O valor é {} do tipo String", a);
}
