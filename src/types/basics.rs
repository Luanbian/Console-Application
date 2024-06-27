pub fn example() {
    let active = true;
    println!("boolean: {}", active);

    let character = 'a';
    println!("char: {}", character);

    let name = "Rust";
    println!("string slice: {}", name);

    let name = String::from("Rust");
    println!("string: {}", name);

    let quantity = 10;
    println!("integer: {}", quantity);

    let price = 10.5;
    println!("float: {}", price);
}