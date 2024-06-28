pub fn one() {
    let name = String::from("Luan");
    show_name(name);
    // println!("{}", name);

    let age = 22;
    show_age(age);

    println!("{}", age);
}

fn show_name(name: String) {
    println!("{}", name);
}

fn show_age(age: u32) {
    println!("{}", age);
}

pub fn two() {
    let name = new_name();
    println!("{}", name);

    let (name, size) = calc_size(name);
    println!("{} tem tamanho {}", name, size)
}

fn new_name() -> String {
    let new_name = String::from("Maria");
    new_name
}

fn calc_size(name: String) -> (String, usize) {
    let size = name.len();
    (name, size)
}