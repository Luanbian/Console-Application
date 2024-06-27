pub fn example() {
    let x = 10;
    let y = 5;

    println!("x => {}, y => {}", x,y);

    if x > y {
        println!("x > y");
    } else if x < y {
        println!("x < y");
    } else {
        println!("x = y");
    }
}