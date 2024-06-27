pub fn example() {
    basic();

    with_parameters(10, String::from("Rust"));

    let r = with_return();
    println!("r #1 => {}", r);

    let r = with_parameters_and_return(8);
    println!("r #2 => {}", r);
}

fn basic() {
    println!("Função básica");
}

fn with_parameters(a: i32, b: String) {
    println!("a => {}", a);
    println!("b => {}", b);
}

fn with_return () -> u32 {
    10
}

fn with_parameters_and_return (a: i32) -> bool {
    if a > 5 {
        return true;
    } else {
        return false;
    }
}